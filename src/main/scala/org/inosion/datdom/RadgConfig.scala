package org.inosion.datdom

import com.typesafe.config._


object RadgContext {

  val config = ConfigFactory.load()

  config.checkValid(ConfigFactory.defaultReference(), "datdom")

  object Settings {

    import scala.collection.JavaConverters._
    def loadString(property:String):String = config.getString(property)
    def getList(property:String):List[String] = config.getStringList(property).asScala.toList

  }

}

