
= API Design DSL Notes

* Java - http://www.infoq.com/articles/internal-dsls-java
* Scala - Might change it to 
  http://www.scala-lang.org/api/current/index.html#scala.Dynamic
  http://stackoverflow.com/questions/10139015/construct-a-list-from-a-series-of-expressions-in-scala
* Groovy -

= Scala API Example

val csvStringArrayData = 
  randomGenerate {
    40 rows {
      col { "colname" name firstname }
      col { "colname" name surname }
      col { "colname" name title }
      col { "colname" date dob }
      col { "colname" number between 1000 and 10000 precision 2 }
      col { "colname" number between 0 and 1 precision 6 }
      col { "colname" address street and number }
      col { "colname" address city }
      col { "colname" address postcode uk }
      col { "colname" address county }
      col { "colname" guid }
      col { "colname" regexp /\d\d\d\d\d\d\d/ }
    }
  }
}

//
// The idea is that Dadagen will autoInxpect your fields, matching a common set of field names
// with preset data generators (eg: dateOfBirth or dob will be a DateOfBirthGenerator)
// if some fields (or all) require a specific rule, the you specify by adding a where field list
// if combining the two parameters (withAutoInspect and where/field) the where field's will override
// and auto-matched ones

// withAutoInspect will not populate a field that it cannot match, rather erring on the side of caution.
// The full list of name automatching generators are
//

val arrayOfObjects = randomGenerate 40 objects from classOf[Person] withAutoInspect where {
  field { "myStringField" .regexgen "[A-Z0-9]{0}"
}
    
  

trait RandomDataGenerator { 

void generate()

}

class ArrayOfStringsOfStringsGenerator {
  
  def rows(numberOf: Int): RandomDataGenerator {

  }

}