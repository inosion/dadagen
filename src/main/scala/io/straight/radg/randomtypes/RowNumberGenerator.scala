package io.straight.radg.randomtypes

import io.straight.radg.Context

/**
 * @author rbuckland
 */
case class RowNumberGenerator(name:String,startingNumber:Int = 1) extends DataGenerator[Int] {

  var num = startingNumber

  override def description: String = "Simple incrementing row number"

  override def internalGenerate(context: Context): Int = {
    val ret = num
    num = num+1
    ret
  }
}
