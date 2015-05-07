package org.inosion.dadagen.randomtypes

import org.inosion.dadagen._

/**
 * @author rbuckland
 */
case class GenericListGenerator(name:String,
                         listName:Option[String],
                         listData:Option[List[String]]
                          ) extends DataGenerator[String] {

  // preload the list if they are using one
  if (listName.isDefined) {
    ListConfigSupport.importConfigListData(listName.get)
  }

  override def description: String = "Generic Random List Generator. Use a configured (*.conf) key.name list or a dynamic" +
    " runtime list"

  override def internalGenerate(context: Context): String = {

    if (listName.isDefined) {
      return ListManager.getRandomValue(listName.get)
    }

    if (listData.isDefined) {
      return RandomUtil.randomFromList(listData.get)
    }

    throw new RadgConfigException("Must supply a list key.name or list of values")

  }
}
