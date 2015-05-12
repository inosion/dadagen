package org.inosion.dadagen.randomtypes

import java.util.Random

import org.inosion.dadagen._
import org.inosion.dadagen.lists.{ListManager, RadgConfigException, ListConfigSupport}

/**
 * @author rbuckland
 */

case class GenericListGenerator(name:String,
                         listName:Option[String] = None,
                         listData:Option[List[String]] = None,
                         weightings:Option[List[Int]] = None
                          )(implicit rand: Random) extends DataGenerator[String] {

  // preload the list if they are using one
  if (listName.isDefined) {
    ListConfigSupport.importConfigListData(listName.get)
  }

  override def description: String = "Generic Random List Generator. Use a configured (*.conf) key.name list or a dynamic" +
    " runtime list"

  override def internalGenerate(context: Context)(implicit rand: Random): String = {

    if (listName.isDefined) {
      return ListManager.getRandomValue(listName.get)
    }

    if (listData.isDefined) {
      weightings match {
        case None => RandomUtil.randomFromList(listData.get)
        case Some(weightings) =>
      }
    }

    throw new RadgConfigException("Must supply a list key.name or list of values")

  }
}
