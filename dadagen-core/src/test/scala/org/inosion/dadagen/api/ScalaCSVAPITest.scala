package org.inosion.dadagen.api

import java.io.StringWriter

import com.fasterxml.jackson.core.`type`.TypeReference
import com.fasterxml.jackson.databind.ObjectWriter
import com.fasterxml.jackson.dataformat.csv.{CsvSchema, CsvMapper}
import org.junit.runner.RunWith
import org.scalatestplus.junit.JUnitRunner
import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.matchers.should.Matchers

/**
 * @author rbuckland
 */
@RunWith(classOf[JUnitRunner])
class ScalaCSVAPITest extends AnyFlatSpec with Matchers {

  "using the scala API" should "create data" in {

  import org.inosion.dadagen.api.scaladsl._

  val generator = dadagen asLists {
          col { "id".rownumber }.
          col { "col title".name title }.
          col { "firstname".name firstname }.
          col { "surname".name surname }.
          col { "int".number between 10 and 1001 }.
          col { "money".number between 1.0 and 10 }.
          col { "gender". gender }.
          col { "random-string".regexgen ("""TEsting [0-9] [a-zA-z_';:"\[\]]{5}""")  }.
          col { "addr_street_line".address street }.
          col { "addr_suburb" .address suburb }.
          col { "addr_city".   address city }.
          col { "addr_district". address district }.
          col { "addr_postcode". address postcode }.
          //col { "list". listFrom ("This row is ${firstname} ${id}") },
          col { "template". template ("This row is ${firstname} ${id}") }
    }

    val header = generator.fieldNames
    val arrayData = generator.generateAll(5)

    // use jackson dataformat to spit the data out

    val mapper = new CsvMapper()
    val csvSchema:CsvSchema = CsvSchema.emptySchema()
      .withoutHeader()
      .withQuoteChar('"')
      .withColumnSeparator(',')
      .withLineSeparator("\n")
    ;
    val writer:ObjectWriter = mapper.writer(csvSchema)

    arrayData.size should be (5)
    print(writer.writeValueAsString(arrayData.map(_.toArray).toArray)  ) // first row

    header.size should be (14) // headers
    // Jackson wants an array
    print(writer.writeValueAsString(header.toArray)  ) // column names
  }
}
