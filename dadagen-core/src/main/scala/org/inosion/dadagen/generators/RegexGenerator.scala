package org.inosion.dadagen.generators

import java.util.Random

import com.mifmif.common.regex.Generex
import org.inosion.dadagen.Context

/**
 * @author rbuckland
 */
case class RegexGenerator(name:String,regex:String)(implicit rand: Random) extends Generator[String] {
  private val generex = new Generex(regex)
  override def internalGenerate(context: Context)(implicit rand: Random): String = generex.random()
}
