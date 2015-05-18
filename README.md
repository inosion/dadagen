# DadaGen - Random Data Generator
![alt tag](https://raw.github.com/inosion/dadagen/master/assets/dadagen-logo.jpg)
## What ? 

A Random Data Generator, with examples and templates to use in a variety of scenarios.

## History

This is the second iteration of a random data library that I have written.
The first, somewhat "pushed and left" version is http://random-data-generator.googlecode.com/

This is a much more feature rich Scala version. Soon I will have the same features as the old.

# So what can it do ?

## Gatling Feeder Support

### As a dependency

```scala
"org.inosion.dadagen" %% "dadagen-core" % "0.2.7"
```

Feeders in Gatling are the method of providing "data" as an iterator, (loading from a CSV for example).
This of course is very easy for dadagen.

First, import the dadagen Scala DSL.

```scala
import org.inosion.dadagen.api.scaladsl._
```
    
Next, create your dadagen defintion (what types of random data you want).
You may also want to define the use of the gatling default ThreadLocal Random
```scala
implicit val rand = scala.concurrent.forkjoin.ThreadLocalRandom.current
val feeder = dadagen asMaps {
    field { "id".rownumber }.
    field { "gender".gender }.
    field { "firstname".name firstname }.
    field { "surname".name surname }.
     // Combine all the values together .. order (what it depends on) does not matter
    field { "message".template("${id} - ${firstname} ${surname} (${gender}) i:${int} ${ref}")}.
    field { "int".number between 10 and 99876 }.
    field { "ref".regexgen("[a-f]{6}-[0-9a-f]{8}") }
} generate() // call generate to make the Iterator
```
Then use the Feeder in your script setup.

```scala
val scn = scenario("scenario1").feed(feeder).exec(http(....))
```

Each "field" name will be a session attribute that you can use.

```scala
 // this would return the "message" which above, is defined as a Template. 
 session.attributes.get("message")
```

If you need to have a "limited" or restricted set of data, you can use the method generateAll

```scala
dadagen asMaps { ... } generateAll(100)
```

instead to generate 100 Map[String,String] entries.

## JMeter Data Provider

Dadagen has been embeded into a Simple JMeter Plugin that allows you to generate random data for your test threads.

You can download the JMeter Plugin Jar from bintray - [JMeter Plugin 0.2.7](https://bintray.com/artifact/download/inosion/maven/org/inosion/dadagen/dadagen-jmeter_2.11/0.2.7/dadagen-jmeter_2.11-0.2.7-assembly.jar)
You will need to put it into you JMETER_HOME/lib/ext

You will then have a new Config Element called "Dadagen Random Data Generator"

Hopefully the configuration is very self explanatory.

[![JMeter Configuration] (assets/jmeter_dadagen_random_gen_ui.png)]


And here is a sample of the resulting "variables"
[![JMeter Vars] (assets/jmeter_dadagen_random_gen_results.png)]

Enjoy!!

## Native Scala Class Creation

Stay Tuned, a native "Class" generator is coming where you specify the class and it will create new instances of it.
Matching each field that it finds to a predefined (convention over configuration) set of named field generators.
(this design comes from the older project http://random-data-generator.googlecode.com/
    
```scala
dadagen.object(classOf[ClassName]).generate()
```

## Native Scala Case Class Creation

As above, we will also auto generate Case Classes.

## Generate a CSV

You can create a CSV with the following snippet of code (using Jackson for CSV generation), or your favourite other CSV generator).

```scala
import org.inosion.dadagen.api.scaladsl._

// note that "col" is the same/synonomous to "field"
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

# Scala DSL and the Generators

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

dadagen has a clever technique of "binding" values on one field, to values in another.
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

## Generic Data Creation

Perhaps the two most important random configurations are "regular expressions" and "templates".

### Regular Expression

The regular expression is from [Generex] (https://github.com/mifmif/Generex)
Some simple examples are

```scala
field { "emailAddress".regexgen """my\.test\.email\.[a-f]{6}\.[0-9]{10}\@foo\.com""" }
...
field { "guid_uuid".regexgen "[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89aAbB][a-f0-9]{3}-[a-f0-9]{12}" }
```

### Templates

Using a template, you can combine "values" together from other fields. It does not matter what order the values are in; so long as you don't make a circlar reference
then is should all work out ok.

```scala
field { "firstname".name firstname }.
field { "surname".name surname }.
field { "full_name".template "${firstname} ${surname}"}
```


## Person Data

### Gender

Male or Female (M or F)
DSL: gender
Scala: GenderGenerator(name,style)
style can be "word" or "char"; M/F or Male/Female

### Name - First Name

An Anglo Saxon Name drawn from a list of 2000, 4000 or 10000 (default is 2000)
DSL: name firstname
Scala: FirstNameGenerator(name,genderFieldName,listName)
genderFieldName = "will default to gender"
listName (a keyed name of the "list" of names to use) 

## Data Lists

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

## Gender

The GenderGenerator(colName) will create a string either "M" or "F".
You can optionally


### And More

Running out of time right now to comment them all. So instead, look at org.inosion.dadagen.randomtypes for the current configured ones. 

# Suggestions?

It is VERY Alpha at the moment but more is coming. Drop me a line or raise an issue and I will attend to it.

Regards
Ramon
