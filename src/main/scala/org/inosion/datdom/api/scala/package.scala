package org.inosion.datdom.api

import org.inosion.datdom.{RandomObjectGenerator, SeqOfSeqOfStringsGenerator}
import org.inosion.datdom.randomtypes._

/**
 * @author rbuckland
 */
package object scala {

  /**
   * Object Generator
   *
   * @param number
   * @param manifest
   * @tparam T
   * @return
   */
//  def randomGenerate[T](number:Int)(implicit manifest: Manifest[T]) =  {
//    val generators: Seq[DataGenerator[_]] = ObjectGeneratorBuilder.from(manifest)
//    RandomObjectGenerator(number,generators).generate()
//  }

  /**
   * CSV Style Data Generator
   * @param listGenerator
   * @return
   */
  def randomGenerate(listGenerator:SeqOfSeqOfStringsGenerator) = listGenerator.generate()

  implicit def intToListOfListOfStringsGenerator(value : Int):RowCountWrappedInt = new RowCountWrappedInt(value)

  def col(generator:DataGenerator[_]):DataGenerator[_] = generator

  class RowCountWrappedInt(int:Int) {
    def rows(generators:DataGenerator[_]*):SeqOfSeqOfStringsGenerator = SeqOfSeqOfStringsGenerator(int,generators)
  }

  class StringWrapperColName(colName:String) {
    def name = new NameGenerators(colName)
    def gender = GenderGenerator(colName)
    def rownumber = RowNumberGenerator(colName)
    def regexgen(regex:String):DataGenerator[String] = RegexGenerator(colName,regex)
    def address = new AddressGenerators(colName)
    def number = new NumberGenerators(colName)
    def template(templateString:String) = TemplateGenerator(colName,templateString)
  }

  implicit def stringToStringWrappedGenerator(colName:String):StringWrapperColName = new StringWrapperColName(colName)

  // Name type Generators
  //  def name = new NameGenerators()
  sealed abstract class WrappedGenerator
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

//  // misc
//  def gender = GenderGenerator("gender")
//  def rownumber = RowNumberGenerator("rownumber")
//  def regexgen(regex:String):DataGenerator[String] = RegexGenerator("regex",regex)

  // number type generators
  //
  class NumberGenerators(generatorName:String) extends WrappedGenerator {
    def between(min:Int) = new NumberMinWrappedInt(generatorName,Left(min))
    def between(min:Double) = new NumberMinWrappedInt(generatorName,Right(min))
  }
  // implicit def intToMinWrapperInt(min : Int):NumberMinWrappedInt = new NumberMinWrappedInt(min)
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
