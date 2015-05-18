package org.inosion.dadagen.api

import org.inosion.dadagen.{ListOfStringsGenerator, MapOfStringsGenerator}
import org.inosion.dadagen.randomtypes._

/**
 * @author rbuckland
 */
package object scaladsl {

  def dadagen = new DadagenWrapper()

  class DadagenWrapper() {
    // gatling uses maps for its session data, as does JMeter
    def asMaps (generators:List[DataGenerator[_]]) = MapOfStringsGenerator(generators)

    // raw CSV data can be made with this one
    def asLists(generators:List[DataGenerator[_]]) = ListOfStringsGenerator(generators)
  }

  /*
   * Used as part of the DSL to denote a field in you "List"
   * col is more suitable to use when working with CSV Data
   */
  def col(generator:DataGenerator[_]):List[DataGenerator[_]] = List(generator)
  def field(generator:DataGenerator[_]):List[DataGenerator[_]] = List(generator)

  implicit class ChainableDataGenerator(generators:List[DataGenerator[_]]) {
    def col(generator:DataGenerator[_]):List[DataGenerator[_]] = generators :+ generator
    def field(generator:DataGenerator[_]):List[DataGenerator[_]] = generators :+ generator
  }

  // Name type Generators
  sealed abstract class WrappedGenerator
  implicit class StringWrapperColName(colName:String) extends WrappedGenerator {
    def name = new NameGenerators(colName)
    def gender = GenderGenerator(colName)
    def rownumber = RowNumberGenerator(colName)
    def regexgen(regex:String):DataGenerator[String] = RegexGenerator(colName,regex)
    def address = new AddressGenerators(colName)
    def number = new NumberGenerators(colName)
    def template(templateString:String) = TemplateGenerator(colName,templateString)
    def list(values:List[String]) = GenericListGenerator(name = colName,listData = Some(values))
    def cfglist(listname:String) = GenericListGenerator(name = colName,listName = Some(listname))
  }

  // sub options when dealing with names
  class NameGenerators(generatorName:String) extends WrappedGenerator {
    def firstname = FirstNameGenerator(generatorName)
    def title = TitleGenerator(generatorName)
    def surname = SurnameGenerator(generatorName)
  }

  // address Generators
 // def address = new AddressGenerators()
  class AddressGenerators(generatorName:String) extends WrappedGenerator {
    def fulladdress = AddressGenerator(generatorName,FullAddress)
    def postcode = AddressGenerator(generatorName,PostCodeOnly)
    def street = AddressGenerator(generatorName,StreetAndNumber)
    def suburb = AddressGenerator(generatorName,SuburbOnly)
    def city = AddressGenerator(generatorName,CityOnly)
    def district = AddressGenerator(generatorName,DistrictOnly)
    def country = AddressGenerator(generatorName,CountryOnly)
  }

  // number type generators
  //
  implicit class NumberGenerators(generatorName:String) extends WrappedGenerator {
    def between(min:Int) = new NumberMinWrappedInt(generatorName,Left(min))
    def between(min:Double) = new NumberMinWrappedInt(generatorName,Right(min))
  }

  class NumberMinWrappedInt(val generatorName:String,min:Either[Int,Double]) {
    def and(max:Int) = {
      min match {
        case Left(minInt) => IntegerGenerator(generatorName,minInt,max)
        case Right(minDbl) => DoubleGenerator(generatorName,minDbl,max)
      }
    }
    def and(max:Double) = DoubleGenerator(generatorName,min.fold( l => l.toDouble, r => r),max)
  }


}
