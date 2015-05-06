package org.inosion.datdom.randomtypes

/**
 * @author rbuckland
 */

sealed abstract class CountryCode {
  def countryCode:String
}
case object CountryCodeGBR extends CountryCode {val countryCode="GBR"}
case object CountryCodeAUS extends CountryCode {val countryCode="AUS"}
case object CountryCodeUSA extends CountryCode {val countryCode="USA"}
case object CountryCodeNZD extends CountryCode {val countryCode="NZD"}
case object CountryCodeIND extends CountryCode {val countryCode="IND"}
