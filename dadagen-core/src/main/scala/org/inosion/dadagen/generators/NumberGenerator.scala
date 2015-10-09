package org.inosion.dadagen.generators

import java.util.Random

import org.inosion.dadagen.Context


/**
 * @author rbuckland
 */
case class IntegerGenerator(name:String,min:Int=Int.MinValue,max:Int=Int.MaxValue)(implicit rand: Random) extends Generator[Int] {
  override def internalGenerate(context: Context)(implicit rand: Random): Int = RandomUtil.randomIntRange(min,max)
}
object IntegerGenerator extends Described {
  val description = "Create a random integer (integer)"
}

case class DoubleGenerator(name:String,min:Double=Double.MinValue,max:Double=Double.MaxValue,precision:Int=2)(implicit rand: Random) extends Generator[Double] {
  override def internalGenerate(context: Context)(implicit rand: Random): Double = RandomUtil.randomDoubleRange(min,max,precision)
}
object DoubleGenerator extends Described {
  val description = "Create a random Double with Optional Precision"
}

case class LongGenerator(name:String,min:Long=Long.MinValue,max:Long=Long.MaxValue)(implicit rand:Random) extends Generator[Long] {
  override def internalGenerate(context: Context)(implicit rand: Random): Long = RandomUtil.randomLongRange(min,max)
}

object LongGenerator extends Described {
  val description = "Create a random Long"
}

/**
 * Support object for creating the right type of "number" generator
 */
object NumberGenerator {
  def createNumberGen(name:String, min:String,max:String,precision:Int):Generator[_] = {
    precision match {
      case 0 => IntegerGenerator(name,min.toInt,max.toInt)
      case _ => DoubleGenerator(name,min.toDouble,max.toDouble,precision)
    }
  }
}
