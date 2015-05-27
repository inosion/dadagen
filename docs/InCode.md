### Generate a CSV

You can create a CSV with the following snippet of code (using Jackson for CSV generation), or your favourite other CSV generator).

```scala
import org.inosion.dadagen.api.scaladsl._

val generator = dadagen asLists {
      field { "id".rownumber }.
      field { "col title".name title }.
      field { "firstname".name firstname }.
      field { "surname".name surname }.
      field { "int".number between 10 and 1001 }.
      field { "money".number between 1.0 and 10 }.
      field { "gender". gender }.
      field { "random-string".regexgen ("""TEsting [0-9] [a-zA-z_';:"\[\]]{5}""")  }.
      field { "addr_street_line".address street }.
      field { "addr_suburb" .address suburb }.
      field { "addr_city".   address city }.
      field { "addr_district". address district }.
      field { "addr_postcode". address postcode }.
      //field { "list". listFrom ("This row is ${firstname} ${id}") },
      field { "template". template ("This row is ${firstname} ${id}") }
}

val header = generator.fieldNames
val arrayData = generator.generateAll(5)

// Create a new Jackson CSV Mapper
val mapper = new CsvMapper()
val csvSchema:CsvSchema = CsvSchema.emptySchema()
  .withoutHeader()
  .withQuoteChar('"')
  .withColumnSeparator(',')
  .withLineSeparator("\n")

val writer:ObjectWriter = mapper.writer(csvSchema)

// Jackson needs an "Array"
print(writer.writeValueAsString(header.toArray)  ) // column names - first (Header) row
print(writer.writeValueAsString(arrayData.map(_.toArray).toArray)  ) // the data
```

