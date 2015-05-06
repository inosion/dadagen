package io.straight.radg.randomtypes

import com.mifmif.common.regex.Generex
import io.straight.radg.Context

/**
 * @author rbuckland
 */
case class RegexGenerator(name:String,regex:String) extends DataGenerator[String] {

  private val generex = new Generex(regex)

  override def internalGenerate(context: Context): String = generex.random()

  override def description: String = "Create a String from a regular expression."

}
