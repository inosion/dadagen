

### Gatling Feeder Support

Gatling support is so simple and very easy (for Dadagen code) ad for you.


#### Setting up the dependency

You only need to include the dadagen-core library in your project.
It is currenty compile for Scala 2.11. If 2.10 is required, we can do that too.

For SBT the Dadgen is hosted on the Bintray Repository. For now, add 
```
https://bintray.com/artifact/download/inosion/maven
```
as your repository

This is the dependency line you need
```scala
"org.inosion.dadagen" %% "dadagen-core" % "0.2.8"
```

#### Coding your Test

An example is found here in the Gatling test [SimpleGatlingFeederExample.scala](dadagen-core/src/test/scala/org/inosion/dadagen/support/gatling/SimpleGatlingFeederExample.scala)

Feeders in Gatling are the method of providing "data" as an iterator, (loading from a CSV for example).
This of course is very easy for Dadagen as we feed, from an iterator, of data to your tests.

First, import the dadagen Scala DSL for Dadagen.

```scala
import org.inosion.dadagen.api.scaladsl._
```
    
Next, create your dadagen definition (what types of random data you want).
You may also want to define the use of the gatling default ThreadLocal Random
```scala

implicit val rand = scala.concurrent.forkjoin.ThreadLocalRandom.current

val dadagenFeeder = dadagen asMaps {
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
val scn = scenario("scenario1").feed(dadagenFeeder).exec(http(....))
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
Instead to generate 100 Map[String,String] entries.

