
= API Design DSL Notes

* Java - http://www.infoq.com/articles/internal-dsls-java
* Scala -
* Groovy -


= Scala API Example

val csvStringArrayData = 
  
    randomGenerate {
      40 rows {
        col 1 { name firstname }
        col 2 { name surname }
        col 3 { name title }
        col 4 { date dob }
        col 5 { number between 1000 and 10000 precision 2 }
        col 6 { number between 0 and 1 precision 6 }
        col 7 { address street and number }
        col 8 { address city }
        col 9 { address postcode uk }
        col 10 { address county }
        col 11 { guid }
        col 12 { regexp /\d\d\d\d\d\d\d/ }
      }
    }
}


val arrayOfObjects = randomGenerate 40 objects from classOf[MyCaseClass]
  

trait RandomDataGenerator { 

void generate()

}

class ArrayOfStringsOfStringsGenerator {
  
  def rows(numberOf: Int): RandomDataGenerator {

  }

}