package org.inosion.dadagen.generators

import java.util.Random

import org.inosion.dadagen.Context
import org.inosion.dadagen.lists.DadagenConfigException

/**
 * This generator will use the value "keyName", a generated "value" from the context, as the
 * key to the map, and then randomly select from the list
 *
 * DependantListGenerator
 * "product_size", "productType",
 *  Map(
 *        "widget1" -> List("9","8","7"),
 *        "widget2" -> List("9","12","15")
 *     )
 *  )
 */
case class DependantListGenerator(name:String,keyName:String,dependantList:Map[String,List[String]]) (implicit rand: Random) extends DataGenerator[String] {

  override val dependencies = List(keyName)

  override val description: String = "Select a value of a predefined list, based on a keyName as the map key"

  override def internalGenerate(context: Context)(implicit rand: Random): String = {

    val m = Map( "d" -> "9", "s" -> "ss")

    val keyValue = context.dataFieldState(keyName).toString
    dependantList.get(keyValue) match {
      case None => throw DadagenConfigException(s"Missing a list with a key name of ${keyValue}")
      case Some(list) => RandomUtil.randomFromList(list).toString
    }

  }
}
