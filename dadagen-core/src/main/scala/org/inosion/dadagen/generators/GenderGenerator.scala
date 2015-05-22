package org.inosion.dadagen.generators

import java.util.Random

import org.inosion.dadagen.Context


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
case class GenderGenerator(name:String, style:String = "char")(implicit rand: Random) extends DataGenerator[String] {
  def description = "Gender: When style is 'char', then single M or F character for Male or Female, or use style:'word'" +
                    " for 'Male' or 'Female'"
  override def internalGenerate(context: Context)(implicit rand: Random):String =
    style match {
      case "char" => RandomUtil.randomFromList[String](List("M","F"))
      case "word" => RandomUtil.randomFromList[String](List("Male", "Female"))
    }

}
