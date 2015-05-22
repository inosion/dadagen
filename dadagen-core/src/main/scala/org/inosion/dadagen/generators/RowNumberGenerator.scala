package org.inosion.dadagen.generators

import java.util.Random

import org.inosion.dadagen.Context

/**
 * @author rbuckland
 */
case class RowNumberGenerator(name:String,startingNumber:Int = 1)(implicit rand: Random) extends DataGenerator[Int] {

  var num = startingNumber

  override def description: String = "Simple incrementing row number"

  override def internalGenerate(context: Context)(implicit rand: Random): Int = {
    val ret = num
    num = num+1
    ret
  }
}
