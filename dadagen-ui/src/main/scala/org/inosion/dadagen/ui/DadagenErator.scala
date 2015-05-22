package org.inosion.dadagen.ui

import java.io.{FileOutputStream, File}

import com.fasterxml.jackson.databind.ObjectWriter
import com.fasterxml.jackson.dataformat.csv.CsvSchema.Builder
import com.fasterxml.jackson.dataformat.csv.{CsvSchema, CsvMapper}
import org.inosion.dadagen.generators.DataGenerator
import org.inosion.dadagen.{ListOfStringsGenerator, RandomDataGenerator, MapOfStringsGenerator}
import org.inosion.dadagen.api.ScalaScriptEngine
import org.slf4j.LoggerFactory

sealed abstract class FileType
case object XmlFile extends FileType
case object CSVFile extends FileType
case object JsonFile extends FileType


case class DadagenCompilationException(msg:String,e:Exception) extends RuntimeException(e)
case class DadagenGeneralException(msg:String,e:Exception) extends RuntimeException(e)

case class DataWriteException(msg:String,e:Exception) extends RuntimeException(e)

object DadagenErator {

  val logger = LoggerFactory.getLogger(DadagenErator.getClass);

  implicit val random = new java.security.SecureRandom

  val engine = ScalaScriptEngine.loadEngine()

  /*
   * This compiles the generator config
   */
  private def createGenerators(dadagenConfig: String) = try {
      engine.eval(
        "import org.inosion.dadagen.api.scaladsl._\n\n\n"
          + dadagenConfig).asInstanceOf[List[DataGenerator[_]]]
    } catch {
      case e: Exception => throw DadagenCompilationException("Compilation Failed", e)
    }

  /*
   * Main method to generate the data and write it out to a file
   */
  def generateData(rows: Int, filetype: FileType, filename: String, dadagenConfig: String) = {

    val startMillis = System.currentTimeMillis

    try {
      val generators = createGenerators(dadagenConfig)

      filetype match {
        case XmlFile => writeXmlFile(generators, filename, rows, startMillis)
        case CSVFile => writeCsvFile(generators, filename, rows, startMillis)
        case JsonFile => writeJsonFile(generators, filename, rows, startMillis)
      }

    } catch {
      case e:DadagenCompilationException => Left(e)
      case e:DataWriteException => Left(e)
      case e:Exception => Left(DadagenGeneralException("An unexpected exception occured", e))
    }
  }

  def writeCsvFile(generators: List[DataGenerator[_]], filename: String, rows: Int, startMillis:Long ):Either[Exception,String] = {

    def buildCsvSchema: CsvSchema = CsvSchema.emptySchema()
      .withQuoteChar('"')
      .withColumnSeparator(',')
      .withLineSeparator("\n")
      .withHeader()

    var ostream: FileOutputStream = null

    try {

      val dadagen = ListOfStringsGenerator(generators)
      val mapper = new CsvMapper()
      val it = dadagen.generate()

      // set all the columns
      val csvSchema = dadagen.fieldNames.foldLeft(buildCsvSchema.rebuild)((b,col) => b.addColumn(col)).build()
      val writer: ObjectWriter = mapper.writer(csvSchema)

      ostream = new FileOutputStream(new File(filename))
      // this closes the stream .. it.take(rows).foreach(x => writer.writeValue(ostream, x.toArray))
      writer.writeValue(ostream, it.take(rows).map(_.toArray).toArray)
      Right(s"Wrote out ${rows} samples to the file: ${filename} in ${(System.currentTimeMillis - startMillis) / 1000} seconds")

    } catch {
      case e:Exception => {
        Left(DataWriteException("Failed to write the data to file", e))
      }
    } finally  {
      ostream.close()
    }
  }

  def writeXmlFile(generators: List[DataGenerator[_]], filename: String, rows: Int, startMillis:Long ):Either[Error,String] = ???

  def writeJsonFile(generators: List[DataGenerator[_]], filename: String, rows: Int, startMillis:Long ):Either[Error,String] = ???

}