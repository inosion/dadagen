package org.inosion.datdom.randomtypes

import com.mifmif.common.regex.Generex
import org.inosion.datdom.Context

/**
 * @author rbuckland
 */
case class RegexGenerator(name:String,regex:String) extends DataGenerator[String] {

  private val generex = new Generex(regex)

  override def internalGenerate(context: Context): String = generex.random()

  override def description: String = "Create a String from a regular expression."

}
