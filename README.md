# What

This is the second iteration of a random data library that I have written.
The first, somewhat "pushed and left" version is http://random-data-generator.googlecode.com/

This is a much more feature rich Scala version.

# So what can it do ?

    // Create CSV / Array of Arrays type Data
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


Populate an Object from Class, returning an Seq of objects which you can later do what you need to

    case class Person(firstname:String,surname:String) 
    val seqObject: Seq[Person] = randomGenerate[Person](1000)

# More to Come

It is VERY Alpha at the moment but more is coming.
