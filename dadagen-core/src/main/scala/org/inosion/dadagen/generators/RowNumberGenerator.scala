package org.inosion.dadagen.generators

import java.util.Random

import org.inosion.dadagen.Context


/**
 * @author rbuckland
 */
case class RowNumberGenerator(name:String,startingNumber:Int = 1)(implicit rand: Random) extends Generator[Int] {

  var num = startingNumber
  override def internalGenerate(context: Context)(implicit rand: Random): Int = {
    val ret = num
    num = num+1
    ret
  }
}
object RowNumberGenerator extends Described {
  val description = "Simple incrementing row number"
}
