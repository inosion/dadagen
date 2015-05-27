## DadaGen - Random Data Generator
![Dadagen Logo](https://raw.github.com/inosion/dadagen/master/assets/dadagen-logo.jpg)
### What is Dadagen ? 

Dadagen is an embedded Scala / Java Library that has a host of simple declarations for creating renaomd data of your
choice. The power lies behind a simple configuration sytax and some helper libraries and plugins for a variety
of scenarios where random, real data is desired.


### History

This is the second iteration of a random data library that I have written.
The first, somewhat "pushed and left" version is http://random-data-generator.googlecode.com/

Dadgen is a much more feature rich Scala version that caters for so much more than the original.

### The Syntax - A Teaser

If you can imagine that we want a CSV of random data, then this definition below will produce that.
It is a very full example of type types of data that can be generated, but we will get to the API and 
Generator types later.

Each row in this CSV will have a value randomly chosen from the "definitions" below.


```scala

  // this import allows us to use the Case Class directly.
  //   field { DoubleGenerator("initial_investment",1000,80000,2) }.   

  import org.inosion.dadagen.generators._

  field { "id".rownumber }.
  field { "r_uuid".regexgen ("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}") } .
  field { "r_rand1".number between 10000 and 90000 }.
  field { "r_str".regexgen ("[A-Z][a-zA-Z]{4}[0-9]{4}") }.
  field { "payload_id".template ("PERFT_${id}_${r_uuid}") }.
  field { "gender".gender }.
  field { "firstname".name firstname }.
  field { "surname_data".name surname }.
  field { "surname".template ("${surname_data}-${r_str}") }.
  field { "fullname".template ("${firstname} ${surname}") }.
  field { "dob".regexgen ("19[3-9][0-9]-(1[012]|0[1-9])-(0[0-9]|1[0-9]|2[0-9])") }.
  field { "email_address".template("TEST_${firstname}.${surname}@noemail.test") }.
  field { "nino".regexgen("(A|B|C|E|G|H|J|K|L|M|N|O|P|R|S|T|W|X|Y|Z){2}[0-9]{6}A") }.
  field { "street_number".number between 1 and 100 }.
  field { "street_name".template ("RS Performance Street" ) }.
  field { "town".address city }.
  field { "postcode".regexgen ("[A-Z][A-Z][0-9] [0-9][A-Z][A-Z]") }.
  // Issue #10 - API is not supporting precision right now, but the case class does (the default _is_ 2.. see "upfront_commission") .. but if you want more precision, this is how
  field { DoubleGenerator("initial_investment",1000,80000,2) }.   
  field { "regular_investment_amount".regexgen("(50|100|150|200|250|300|350|400|450|500|550|600|650|700|750|800|850|900|950)") }.
  field { "account_number".number between 8800000 and 8899999 }.
  field { "sort_code".regexgen("(402205|110124|830608|880011|938424|938343|938130)") }.
  field { "mobile_phone_number".regexgen ("07777 [0-9]{3} [0-9]{3}") }.
  field { "retirement_age".number between 65 and 75 }.
  field { "upfront_commission".number between 100.00 and 150.00 }. // when using floats, the default is precision 2 (that is, this will create eg 110.18 ) 
  field { "commission_percentage".number between 0.01 and 0.05 }
```

### Where Can it be Used ?

We have support for various test suites and situations

* [Gatling Feeder Support](docs/Gatling.md)
* Native JMeter Plugin [JMeter Plugin](docs/JMeter.md)
* A Helpful Standalone Java GUI App for making CSVs, XML and JSON files [Dadagen UI](docs/DadagenUI.md)
* An example of using it with Silk Performer [Dadagen + SilkPerformer](docs/SilkPerformer.md)
* For Testing *anything* with Java [Java API Usage](docs/JavaUsage.md)
* For Testing *anything* with Scala [Scala API USage](docs/ScalaUsage.md)
* Some examples of Embedding it into your Application for CSV, XML, or JSON generation [Code Usage](docs/InCode.md)

## Native Scala Class Creation

Stay Tuned, a native "Class" generator is coming where you specify the class and it will create new instances of it.
Matching each field that it finds to a predefined (convention over configuration) set of named field generators.
(this design comes from the older project http://random-data-generator.googlecode.com/
    
```scala
dadagen.object(classOf[ClassName]).generate()
```
### The Dadagen DSL and the Generators

A Small note about the DSL. 
The keyword "col" or "field" is just syntatic sugar to accept a Generator and concatenating them into a list.
The Generators are the heart of dadagen.

```scala
col { "fieldname" name firstname } 
```

is equivalent to 

```scala
col { FirstNameGenerator(fieldName) }
```
    
So you can write your own generator and drop it in place
    
```scala
col { MyCustomGenerator(someFieldName) }
```

Just extend 

```scala
DataGenerator[ T ]
```

# Context / Binding

Dadagen has a clever technique of "binding" values on one field, to values in another.
For example, if you include "gender" with "firstname", then dadagen knows to only supply Female names when the Gender is female
and similarly, Male names when the Gender is Male.

Future use will be around Country binding (only provide UK Tax File Numbers (NINO) when the Country is UK, USA when USA etc).
In addition, Country binding can be used in Address Styles etc.. (so that is to come.. infrastructure is there)

# Data Types

You will have noticed, by looking at the above examples that there is a lot of "sample" data embedded.
This is the core idea of dadagen, that we can provide all the data you need. 

Each embedded data type is supported by a simple Case Class Generator. So you can both write your own generator
and mix and match other ones to your hearts content.

The lists are defined in the dadagen config file (src/main/resources/reference.conf)

The lists can be used arbitrarily, outside of a defined "Configured Generator" and you can even add you own lists by extending the Typesafe Config.

The current lists defined are

!inc(src/main/resources/reference.conf)

### Generic Data Creation - The Generators

Behind Dadagen are sets of Generators create random data based on configuration.
 

Perhaps the two most important random configurations are "regular expressions" and "templates".

Below is a very full example of type types of data that can be generated.
```scala

  // this import allows us to use the Case Class directly.
  //   field { DoubleGenerator("initial_investment",1000,80000,2) }.   

  import org.inosion.dadagen.generators._

  field { "id".rownumber }.
  field { "r_uuid".regexgen ("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}") } .
  field { "r_rand1".number between 10000 and 90000 }.
  field { "r_str".regexgen ("[A-Z][a-zA-Z]{4}[0-9]{4}") }.
  field { "payload_id".template ("PERFT_${id}_${r_uuid}") }.
  field { "gender".gender }.
  field { "firstname".name firstname }.
  field { "surname_data".name surname }.
  field { "surname".template ("${surname_data}-${r_str}") }.
  field { "fullname".template ("${firstname} ${surname}") }.
  field { "dob".regexgen ("19[3-9][0-9]-(1[012]|0[1-9])-(0[0-9]|1[0-9]|2[0-9])") }.
  field { "email_address".template("TEST_${firstname}.${surname}@noemail.test") }.
  field { "nino".regexgen("(A|B|C|E|G|H|J|K|L|M|N|O|P|R|S|T|W|X|Y|Z){2}[0-9]{6}A") }.
  field { "street_number".number between 1 and 100 }.
  field { "street_name".template ("RS Performance Street" ) }.
  field { "town".address city }.
  field { "postcode".regexgen ("[A-Z][A-Z][0-9] [0-9][A-Z][A-Z]") }.
  // Issue #10 - API is not supporting precision right now, but the case class does (the default _is_ 2.. see "upfront_commission") .. but if you want more precision, this is how
  field { DoubleGenerator("initial_investment",1000,80000,2) }.   
  field { "regular_investment_amount".regexgen("(50|100|150|200|250|300|350|400|450|500|550|600|650|700|750|800|850|900|950)") }.
  field { "account_number".number between 8800000 and 8899999 }.
  field { "sort_code".regexgen("(402205|110124|830608|880011|938424|938343|938130)") }.
  field { "mobile_phone_number".regexgen ("07777 [0-9]{3} [0-9]{3}") }.
  field { "retirement_age".number between 65 and 75 }.
  field { "upfront_commission".number between 100.00 and 150.00 }. // when using floats, the default is precision 2 (that is, this will create eg 110.18 ) 
  field { "commission_percentage".number between 0.01 and 0.05 }
```

#### Regular Expression

The regular expression is from [Generex] (https://github.com/mifmif/Generex)
Some simple examples are

```scala
field { "emailAddress".regexgen ("""my\.test\.email\.[a-f]{6}\.[0-9]{10}\@foo\.com""") }
...
field { "guid_uuid".regexgen ("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89aAbB][a-f0-9]{3}-[a-f0-9]{12}") }
```

#### Templates

Using a template, you can combine "values" together from other fields. It does not matter what order the values are in; so long as you don't make a circlar reference
then is should all work out ok.

```scala
field { "firstname".name firstname }.
field { "surname".name surname }.
field { "full_name".template "${firstname} ${surname}"}
```


#### Person Data - Gender

Male or Female (M or F)
DSL: gender
Scala: GenderGenerator(name,style)
style can be "word" or "char"; M/F or Male/Female

#### Person Data - Name - First Name

An Anglo Saxon Name drawn from a list of 2000, 4000 or 10000 (default is 2000)
DSL: name firstname
Scala: FirstNameGenerator(name,genderFieldName,listName)
genderFieldName = "will default to gender"
listName (a keyed name of the "list" of names to use) 

#### Data Lists

A lot of the Data Generators use a custom "list" of data to provide real test data. For example a list of firstnames is available.
There are two ways to use a list of values. 

1. Provide a static (fixed list)

```scala
field { "mycustomlist".list List("blue","green","pink","dust red","black") }
```

2. Use a file : configure it in a config file, put it on the classpath

This is more advanced (not that it is hard)
You will need to configure the list in an "application.conf" file in your classpath.

```scala
dadagen {
  lists {
    myorg { 
      mycustomlist.filename = "classpath:somecustomlist.csv"
      filesystemlist.filename = "file:../some/../relative/path/someother.csv"

      glass {
        values = ["Chevron","Beveled","Flat","Float","Ground","Silica fiber","Starfire","Frosted","Rippled"]
      }
    }
  }
}

... 
// using the list
field { "mycustomlist".cfglist "myorg.mycustomlist" }.
field { "glass".cfglist "myorg.glass" }.
```

To see more example of pre-configured lists (ones that we have bundled in the Jar), see the Typesafe Config file for Dadagen in [resources.conf](https://github.com/inosion/dadagen/blob/master/dadagen-core/src/main/resources/reference.conf)

If there is a list of "stuff" that you think is really helpful, (like a list of average rain drop sizes :-) ) then create a issue with the list, or a pull request and we'll add it in.

#### Gender

The GenderGenerator(colName) will create a string either "M" or "F".
You can optionally


#### And More

Running out of time right now to comment them all. So instead, look at org.inosion.dadagen.generators for the current configured ones. 

### Suggestions?

It is VERY Alpha at the moment but more is coming. Drop me a line or raise an issue and I will attend to it.

Regards
Ramon
