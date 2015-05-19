package org.inosion.dadagen.lists

import java.io.{File, FileInputStream, InputStream}

import com.typesafe.config.{ConfigException, ConfigValue}
import org.inosion.dadagen.RadgContext
import org.slf4j.LoggerFactory

import scala.collection.JavaConversions._
import scala.collection.JavaConverters._
import scala.collection._
/**
 * All our data starts as CSV file from property files
 *
 * We will load it to an in memory H2 database.
 * this might be overkill because some of the lists are small
 * We want a way to pull data on discriminators
 *
 * TODO perhaps change this to Map List of Lists with Map of Discriminators to List Indexes
 *
 *
 * @author rbuckland
 */
object ListConfigSupport {

  val keyPrefix = "dadagen.lists."

  val suffixMatchReg = "(filename|values)\\.?(.*?)$"

  val logger = LoggerFactory.getLogger(ListConfigSupport.getClass);


  def importConfigListData(listKeyName:String):Unit = {

    val keyConfig = RadgContext.config.getConfig(keyPrefix + listKeyName)

    import JavaConversions._
    logger.trace(keyConfig.entrySet().iterator().mkString(","))

    // we could have three types of entries
    // filename
    // values = ["array"]
    // values { foo = ["array"], bar = ["array"]

    if (keyConfig.hasPath("filename")) {

      importFile(listKeyName,keyConfig.getString("filename"))

    } else if (keyConfig.hasPath("values")) {

      // it is either
      // values = ["array"]
      //     or
      // values { foo = ["array"], bar = ["array"]  }

      try {
        val config = keyConfig.getConfig("values")
        // we have multiple discriminators
        for (
          discrim: java.util.Map.Entry[String, ConfigValue] <- config.entrySet().iterator()
        ) {
          val list:java.util.List[String] = discrim.getValue.unwrapped().asInstanceOf[java.util.List[String]]
          ListManager.importDataWithDiscriminator(listKeyName,list.asScala,discrim.getKey)
        }
      } catch {
        // it was not a Config subtype
        case e: ConfigException.WrongType => {
          ListManager.importData(listKeyName
            ,keyConfig.getStringList("values").asScala.map{x => List(x)}.toStream
            ,false)
        }
      }

    } else {
      throw new DadagenConfigException(s"The key ${keyPrefix + listKeyName + ".values"} was not found in the configuration")
    }

  }

  def importFile(listKeyName:String, filename:String): Unit = {

      val classpathre = "^(classpath|file):(.*)$".r
      classpathre.findFirstMatchIn(filename) match {
        case None => throw new RuntimeException(s"Url for the file was not recognised: $filename")
        case Some(m) => m.group(1) match {

          case "classpath" =>
            importStreamWithLogging(listKeyName,m.group(2),getClass.getClassLoader.getResourceAsStream(m.group(2)))

          case "file" =>
            importStreamWithLogging(listKeyName,m.group(2),new FileInputStream(new File(m.group(2))))
        }
      }

      def importStreamWithLogging(listKeyName:String, filepath:String, inputStream:InputStream) = {
        // do a check on Listmanager .. it checks also .. but we will here so we get better logging
        //
        if (! ListManager.listData.single.containsKey(listKeyName)) {
          logger.info(s"Importing $listKeyName from $filepath")
          val now = System.currentTimeMillis()
          ListManager.importFile(listKeyName, inputStream)
          val milliseconds = (System.currentTimeMillis() - now)
          logger.info(s"Finished importing in (${milliseconds}ms) $listKeyName from $filepath")
        }
      }

  }

}

// simple case class for the config of a list
case class ListConfiguration(key:String,listName:String,listType:String)


case class DadagenConfigException(message:String) extends RuntimeException(message)
