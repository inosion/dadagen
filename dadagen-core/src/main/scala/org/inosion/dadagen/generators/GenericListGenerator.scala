package org.inosion.dadagen.generators

import java.util.Random

import org.inosion.dadagen._
import org.inosion.dadagen.lists.{ListManager, DadagenConfigException, ListConfigSupport}

/**
 * @author rbuckland
 */

case class GenericListGenerator(name:String,
                         listName:Option[String] = None,
                         listData:Option[List[String]] = None,
                         weightings:Option[List[Int]] = None
                          )(implicit rand: Random) extends Generator[String] {

  // preload the list if they are using one
  if (listName.isDefined) {
    ListConfigSupport.importConfigListData(listName.get)
  }

  override def internalGenerate(context: Context)(implicit rand: Random): String = {

    if (listName.isDefined) {
      return ListManager.getRandomValue(listName.get)
    }

    if (listData.isDefined) {
      weightings match {
        case None => return RandomUtil.randomFromList(listData.get)
        case Some(weightings) =>
      }
    }

    throw new DadagenConfigException("Must supply a list key.name or list of values")

  }
}

object GenericListGenerator extends Described {
  override val description = "Generic Random List Generator. Use a configured (*.conf) key.name list or a dynamic" +
    " runtime list"
}
