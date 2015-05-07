package org.inosion.dadagen.randomtypes

import com.mifmif.common.regex.Generex
import org.inosion.dadagen.{ListConfigSupport, ListManager, RandomUtil, Context}
import ListManager.getRandomValue

/*
 * Enums for Driving what Style we want
 */
sealed abstract class AddressStyle
case object FullAddress extends AddressStyle
case object StreetAndNumber extends AddressStyle
case object StreetOnly extends AddressStyle
case object SuburbOnly extends AddressStyle
case object SuburbCity extends AddressStyle
case object CityOnly extends AddressStyle
case object DistrictOnly extends AddressStyle
case object SuburbCityDistrict extends AddressStyle
case object CountryOnly extends AddressStyle
case object PostCodeOnly extends AddressStyle

/**
 * TODO add weighted house name and suffixes
 * TODO add binding to country style addresses
 * @param name
 * @param style
 */
case class AddressGenerator(name:String,
                                       style:AddressStyle,
                                       countryField:Option[CountryCode] = Some(CountryCodeGBR)
                                        ) extends DataGenerator[String] {

  val ukPostcodeGen = new Generex("[A-HJ-NP-Z]{2}[1-9][0-9] [0-9][A-HJ-NP-Z]{2}") // not proper but close

  val streetNameKey = "address.street.name"
  val houseNameKey = "address.housename"
  val streetSuffixKey = "address.street.suffix"
  val addressSuburbKey = "address.suburb"
  val addressDistrictKey = "address.district"
  val placesCitiesKey = "places.cities"
  val countryKey = "places.countries"

  ListConfigSupport.importConfigListData(houseNameKey)
  ListConfigSupport.importConfigListData(streetNameKey)
  ListConfigSupport.importConfigListData(streetSuffixKey)
  ListConfigSupport.importConfigListData(addressSuburbKey)
  ListConfigSupport.importConfigListData(addressDistrictKey)
  ListConfigSupport.importConfigListData(placesCitiesKey)
  ListConfigSupport.importConfigListData(countryKey)

  override def description: String = "Generate lots of Address Stuff suburb and city and county (state, district) style address'"

  override def internalGenerate(context: Context): String = {

    style match {
      case StreetAndNumber => streetLine
      case StreetOnly => streetName
      case SuburbCityDistrict => suburbCityDistrict
      case SuburbOnly => getRandomValue(addressSuburbKey)
      case CityOnly => getRandomValue(placesCitiesKey)
      case DistrictOnly => getRandomValue(addressDistrictKey)
      case SuburbCity => suburbCity
      case PostCodeOnly => postcode
      case CountryOnly => getRandomValue(countryKey)
      case FullAddress => streetLine + ", " + suburbCityDistrict +  " " + getRandomValue(countryKey).toUpperCase() +
        " " + postcode
    }
  }

  private def suburbCity =  getRandomValue(addressSuburbKey) + ", " + getRandomValue(placesCitiesKey)
  private def suburbCityDistrict = suburbCity + ", " + getRandomValue(addressDistrictKey)
  private def streetName = getRandomValue(streetNameKey) + " " + getRandomValue(streetSuffixKey)
  private def streetLine =  houseNameNumber + " " + streetName
  private def houseNameNumber:String = {
    RandomUtil.randomIntUpto(20) match {
      case 1 => getRandomValue(houseNameKey) + "," // 5% of time
      case _ => RandomUtil.randomIntUpto(500) + (
        RandomUtil.randomIntUpto(40) match { // 2.5% of time is a "10A or 100B"
        case 1 => RandomUtil.randomFromList(Seq("A","B"))
        case _ => ""
      })

    }
  }
  private def postcode:String = countryField match {
    case Some(CountryCodeGBR) => ukPostcodeGen.random()
    case _ =>  RandomUtil.randomIntRange(55000,56000).toString
  }
  private def ukPostcode = ukPostcodeGen.random()
}
