package org.inosion.dadagen.generators

import java.text.{SimpleDateFormat, DateFormat}
import java.util.Date
import java.util.Random

import org.inosion.dadagen.Context

/**
 * Create a date of birth. The defaults will be someone less than 100 yo.
 */
case class DobGenerator(name: String, yearOffset:Int = 100)(implicit rand: Random) extends DataGenerator[Date] {

  override def description = "A Date of Birth Generator. Defaults to 100 years"

  override def internalGenerate(context: Context)(implicit rand: Random): Date = {
    val currentSeconds = new Date().getTime
    val startingSeconds = new Date().getTime - (60 * 60 * 24 * 365 * yearOffset)
    new DobDate(RandomUtil.randomLongRange(startingSeconds, currentSeconds))
 }
}

class DobDate(s:Long ) extends Date(s) {
  override def toString: String = {
    val df = new SimpleDateFormat("yyyy-mm-dd")
    df.format(this)
  }
}
