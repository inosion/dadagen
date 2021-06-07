package org.inosion.dadagen

import org.inosion.dadagen.generators._

package object auto {

  implicit val rand: java.util.Random = org.inosion.dadagen.rand

  def dadagen[T]:DadagenPrep[T] = ??? // new DadagenPrep[T]()

  implicit class DadagenPrep[T](generators:List[Generator[_]]) extends DadagenBuilder[T] {
    def makeGenerator(): Dadagenerator[T] = ???
  }

  implicit def convert[T](p: DadagenPrep[T]):Dadagenerator[T] = p.makeGenerator()

  private[this] def fieldToGenerator[F](fieldName: String, tipe: F, className:String) = {
    val f = s"${className}id"

    (tipe, fieldName.toLowerCase) match {

      // primary ID types
      case (Int,"id"|"primaryid") => IntegerGenerator(fieldName)
      case (Long,"id"|"primaryid") => LongGenerator(fieldName)


      // names
      case (f:String,"firstname"|"christianname") => FirstNameGenerator(fieldName)
      case (f:String,"surname"|"lastname"|"familyname") => SurnameGenerator(fieldName)
      case (f:String,"title") => TitleGenerator(fieldName)
      case (f:String,"gender"|"sex") => GenderGenerator(fieldName)

      // address -- these are hard (to find good matches)
      case (f:String,"country") => AddressGenerator(fieldName,CountryOnly)
      case (f:String,"address") => AddressGenerator(fieldName,FullAddress)
      case (f:String,"streetaddress"|"streetline"|"addressline1") => AddressGenerator(fieldName,StreetWithNumber)
      case (f:String,"addressline3") => AddressGenerator(fieldName,StreetWithNumber)

      // misc
      case (f:String,"uuid"|"guid"|"guuid") => MiscGenerators.uuidGenerator.copy(name=fieldName)
    }
  }
}
