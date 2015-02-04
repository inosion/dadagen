package io.straight.radg

import com.typesafe.config._


object RadgContext {

  val config = ConfigFactory.load()

  config.checkValid(ConfigFactory.defaultReference(), "radg")

  object Settings {

    import scala.collection.JavaConverters._
    def loadString(property:String):String = config.getString(property)
    def getList(property:String):List[String] = config.getStringList(property).asScala.toList

  }

}

