package io.straight.radg.randomtypes

import io.straight.radg.Context
import io.straight.radg.RandomUtil


/*
 * Not really using this complexity .. just Char M or F
 */
/*
sealed abstract class Gender {
  def gender:String
  val genderChar = gender.charAt(0)
  val genderUpper = gender.toUpperCase
  override def toString = gender
}

case object Female extends Gender {
  val gender = "Female"
}
case object Male extends Gender  {
  val gender = "Male"
}
*/

/**
 * @author rbuckland
 */
case class GenderGenerator(name:String, style:String = "char") extends DataGenerator[String] {
  def description = "Gender: When style is 'char', then single M or F character for Male or Female, or use style:'word'" +
                    " for 'Male' or 'Female'"
  override def internalGenerate(context: Context):String =
    style match {
      case "char" => RandomUtil.randomFromList[String](List("M","F"))
      case "word" => RandomUtil.randomFromList[String](List("Male", "Female"))
    }

}
