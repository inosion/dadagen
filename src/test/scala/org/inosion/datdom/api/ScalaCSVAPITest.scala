package org.inosion.datdom.api

import java.io.StringWriter

import com.fasterxml.jackson.core.`type`.TypeReference
import com.fasterxml.jackson.databind.ObjectWriter
import com.fasterxml.jackson.dataformat.csv.{CsvSchema, CsvMapper}
import org.inosion.datdom.api.scala._
import org.scalatest.{Matchers, FlatSpec}


/**
 * @author rbuckland
 */
class ScalaCSVAPITest extends FlatSpec with Matchers {

  "using the scala API" should "create data" in {

  import org.inosion.datdom.api.scala._

  val arrayData = randomGenerate {
      1000 rows (
          col { "id".rownumber },
          col { "col title".name title },
          col { "firstname".name firstname },
          col { "surname".name surname },
          col { "int".number between 10 and 1001 },
          col { "money".number between 1.0 and 10 },
          col { "gender". gender },
          col { "random-string".regexgen ("""TEsting [0-9] [a-zA-z_';:"\[\]]{5}""")  },
          col { "addr_street_line".address street },
          col { "addr_suburb" .address suburb },
          col { "addr_city".   address city },
          col { "addr_district". address district },
          col { "addr_postcode". address postcode },
          //col { "list". listFrom ("This row is ${firstname} ${id}") },
          col { "template". template ("This row is ${firstname} ${id}") }
        )
    }


    // use jackson dataformat to spit the data out

    val mapper = new CsvMapper()
    val csvSchema:CsvSchema = CsvSchema.emptySchema()
      .withoutHeader()
      .withQuoteChar('"')
      .withColumnSeparator(',')
      .withLineSeparator("\n")
    ;
    val writer:ObjectWriter = mapper.writer(csvSchema)

    println(arrayData)
    // Jackson wants an array
    print(writer.writeValueAsString(arrayData._1.toArray)  ) // column names
    print(writer.writeValueAsString(arrayData._2.map(_.toArray).toArray)  ) // first row
  }
}
