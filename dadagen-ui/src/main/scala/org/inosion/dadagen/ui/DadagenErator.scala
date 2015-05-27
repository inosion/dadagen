package org.inosion.dadagen.ui

import java.io.{FileOutputStream, File}

import com.fasterxml.jackson.databind.{ObjectMapper, ObjectWriter}
import com.fasterxml.jackson.dataformat.csv.{CsvSchema, CsvMapper}
import com.fasterxml.jackson.dataformat.xml.XmlMapper
import com.fasterxml.jackson.module.scala.DefaultScalaModule
import com.fasterxml.jackson.module.scala.experimental.ScalaObjectMapper
import org.inosion.dadagen.generators.DataGenerator
import org.inosion.dadagen.{MapOfStringsGenerator, ListOfStringsGenerator}
import org.inosion.dadagen.api.ScalaScriptEngine
import org.slf4j.LoggerFactory

import scala.collection.JavaConversions._

sealed abstract class FileType(val name:String) {
  override def toString = name
  val extension = name.toLowerCase
}
case object XmlFile extends FileType("XML")
case object CSVFile extends FileType("CSV")
case object JsonFile extends FileType("JSON")


case class DadagenCompilationException(msg:String,e:Exception) extends RuntimeException(e)
case class DadagenGeneralException(msg:String,e:Exception) extends RuntimeException(e)

case class DataWriteException(msg:String,e:Exception) extends RuntimeException(e)

object DadagenErator {

  val logger = LoggerFactory.getLogger(DadagenErator.getClass)

  implicit val random = new java.security.SecureRandom

  val engine = ScalaScriptEngine.loadEngine
  lazy val xmlMapper = new XmlMapper() with ScalaObjectMapper
  lazy val jsonMapper = new ObjectMapper() with ScalaObjectMapper
  lazy val csvMapper = new CsvMapper() with ScalaObjectMapper

  /*
   * This compiles the generator config
   */
  private def createGenerators(dadagenConfig: String) = try {
      engine.eval(
        "import org.inosion.dadagen.api.scaladsl._\n\n" + dadagenConfig
      ).asInstanceOf[List[DataGenerator[_]]]
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
      val ostream = new FileOutputStream(new File(filename))

      filetype match {
        case XmlFile => writeXmlFile(generators, ostream, rows)
        case CSVFile => writeCsvFile(generators, ostream, rows)
        case JsonFile => writeJsonFile(generators, ostream, rows)
      }

      ostream.close()

      Right(s"Wrote $rows sample${if(rows>1)"s"} to: $filename in ${(System.currentTimeMillis - startMillis) / 1000} seconds")

    } catch {
      case e:DadagenCompilationException => Left(e)
      case e:DataWriteException => Left(e)
      case e:Exception => Left(DadagenGeneralException("An unexpected exception occured", e))
    }
  }

  /*
   * Write out a CSV File
   */
  private def writeCsvFile(generators: List[DataGenerator[_]], ostream: FileOutputStream, rows: Int)= {

    def buildCsvSchema: CsvSchema = CsvSchema.emptySchema()
      .withQuoteChar('"')
      .withColumnSeparator(',')
      .withLineSeparator("\n")
      .withHeader()

    val dadagen = ListOfStringsGenerator(generators)

    // set all the columns
    val csvSchema = dadagen.fieldNames.foldLeft(buildCsvSchema.rebuild)((b,col) => b.addColumn(col)).build()
    val writer: ObjectWriter = csvMapper.writer(csvSchema)
    val it = dadagen.generate()

    // this closes the stream .. it.take(rows).foreach(x => writer.writeValue(ostream, x.toArray))
    writer.writeValue(ostream, it.take(rows).map(_.toArray).toArray)
  }

  /*
   * Write out an XML File
   */
  private def writeXmlFile(generators: List[DataGenerator[_]], ostream: FileOutputStream, rows: Int) = {

      xmlMapper.registerModule(DefaultScalaModule)
      val dadagen = MapOfStringsGenerator(generators)
      xmlMapper.writeValue(ostream,dadagen.generateAll(rows))

  }

  /*
   * Write out a JSON file
   */
  private def writeJsonFile(generators: List[DataGenerator[_]], ostream: FileOutputStream, rows: Int): Unit = {

    jsonMapper.registerModule(DefaultScalaModule)
    val dadagen = MapOfStringsGenerator(generators)
    jsonMapper.writeValue(ostream,dadagen.generateAll(rows))

  }

}