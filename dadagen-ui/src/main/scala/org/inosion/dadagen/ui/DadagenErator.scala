package org.inosion.dadagen.ui

import java.io.{FileOutputStream, File}

import com.fasterxml.jackson.databind.{SerializationFeature, ObjectMapper, ObjectWriter}
import com.fasterxml.jackson.dataformat.csv.{CsvSchema, CsvMapper}
import com.fasterxml.jackson.dataformat.xml.{JacksonXmlModule, XmlMapper}
import com.fasterxml.jackson.module.scala.{DefaultScalaModule}
import com.fasterxml.jackson.module.scala.experimental.ScalaObjectMapper
import org.inosion.dadagen.generators.TypedDataGenerator$
import org.inosion.dadagen.{MapOfStringsGenerator, ListOfStringsGenerator}
import org.inosion.dadagen.api.ScalaScriptEngine
import org.slf4j.LoggerFactory
import scala.collection.JavaConverters._


sealed abstract class FileType(val name: String) {
  override def toString = name

  val extension = name.toLowerCase
}

case object XmlFile extends FileType("XML")

case object CSVFile extends FileType("CSV")

case object JsonFile extends FileType("JSON")


case class DadagenCompilationException(msg: String, e: Exception) extends RuntimeException(e)

case class DadagenGeneralException(msg: String, e: Exception) extends RuntimeException(e)

case class DataWriteException(msg: String, e: Exception) extends RuntimeException(e)

object DadagenErator {

  val logger = LoggerFactory.getLogger(DadagenErator.getClass)

  implicit val rand = new java.security.SecureRandom

  val engine = ScalaScriptEngine.loadEngine

  lazy val xmlMapper = {
    val mod = new JacksonXmlModule()
    mod.setDefaultUseWrapper(false)
    val xml = new XmlMapper(mod) with ScalaObjectMapper
    // Indentation is failing, even though woodstox is included
    // error: com.fasterxml.jackson.core.JsonGenerationException: Underlying Stax XMLStreamWriter (of type com.ctc.wstx.sw.RepairingNsStreamWriter) does not implement Stax2 API natively and is missing method 'writeRaw': this breaks functionality such as indentation that relies on it. You need to upgrade to using compliant Stax implementation like Woodstox or Aalto
    // xml.configure(SerializationFeature.INDENT_OUTPUT, true)
    xml
  }
  lazy val jsonMapper: ObjectMapper = new ObjectMapper() with ScalaObjectMapper
  lazy val csvMapper = new CsvMapper() with ScalaObjectMapper

  /*
   * This compiles the generator config
   */
  private def createGenerators(dadagenConfig: String) = try {
    engine.eval(
      "import org.inosion.dadagen.api.scaladsl._\n\n" + dadagenConfig
    ).asInstanceOf[List[TypedDataGenerator[_]]]
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

      s"Wrote $rows sample${if (rows > 1) "s"} to: $filename in ${(System.currentTimeMillis - startMillis) / 1000} seconds"

    } catch {
      case e: DadagenCompilationException => throw e
      case e: DataWriteException => throw e
      case e: Exception => throw DadagenGeneralException("An unexpected exception occured", e)
    }
  }

  /*
   * Write out a CSV File
   */
  private def writeCsvFile(generators: List[TypedDataGenerator[_]], ostream: FileOutputStream, rows: Int) = {

    def buildCsvSchema: CsvSchema = CsvSchema.emptySchema()
      .withQuoteChar('"')
      .withColumnSeparator(',')
      .withLineSeparator("\n")
      .withHeader()

    val dadagen = ListOfStringsGenerator(generators)

    // set all the columns
    val csvSchema = dadagen.fieldNames.foldLeft(buildCsvSchema.rebuild)((b, col) => b.addColumn(col)).build()
    val writer: ObjectWriter = csvMapper.writer(csvSchema)
    val it = dadagen.generate()

    // this closes the stream .. it.take(rows).foreach(x => writer.writeValue(ostream, x.toArray))
    writer.writeValue(ostream, it.take(rows).map(_.toArray).toArray)
  }

  /*
   * Write out an XML File
   */
  private def writeXmlFile(generators: List[TypedDataGenerator[_]], ostream: FileOutputStream, rows: Int) = {

    xmlMapper.registerModule(DefaultScalaModule)
    val dadagen = MapOfStringsGenerator(generators)
    val list = dadagen.generateAll(rows).map(m => m.asJava).asJava
    xmlMapper.writeValue(ostream, XmlData(list))

  }

  /*
   * Write out a JSON file
   */
  private def writeJsonFile(generators: List[TypedDataGenerator[_]], ostream: FileOutputStream, rows: Int): Unit = {

    jsonMapper.registerModule(DefaultScalaModule)
    val dadagen = MapOfStringsGenerator(generators)
    jsonMapper.writerWithDefaultPrettyPrinter[ObjectWriter]().writeValue(ostream, dadagen.generateAll(rows))

  }

}

case class XmlData(item: java.util.List[java.util.Map[String, String]])