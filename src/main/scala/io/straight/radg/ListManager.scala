package io.straight.radg


import java.io.InputStream

import com.fasterxml.jackson.databind.MappingIterator
import com.fasterxml.jackson.dataformat.csv.{CsvParser, CsvMapper}
import org.slf4j.LoggerFactory

import scala.concurrent.stm._
import scala.collection.JavaConversions._

/**
 * STM MANAGED MAP of List values that the random code can pull from
 * @author rbuckland
 */
object ListManager {

  val logger = LoggerFactory.getLogger(ListManager.getClass);

  // listName and the data
  val listData:TMap[String,Seq[String]] = TMap.empty

  // index is the listname,discriminator and the index to the "array positions" for the list in listData
  // we are only supporting one discriminator key per list. (just easier today)
  case class IndexKey(listName:String,discriminator:String)

  val index:TMap[IndexKey,Seq[Int]] = TMap.empty

  /**
   * Import row data. If there is a descriminator expected, we will use that to create an index
   * over rows by discriminator type
   *
   * The format of the rows data is expected to be
   *
   * rows [   ["Name","M"],
   *          ["Name","M"],
   *          ["Name","F"]
   *       ]
   *
   * @param listName
   * @param rows
   * @param hasDiscriminator
   */
  def importData(listName:String,rows:java.util.Iterator[Array[String]], hasDiscriminator:Boolean):Unit = {

    // for the import of data, we will only do this once. By list name
    if (listData.single.containsKey(listName)) {
      // logger.trace(s"Not importing [${listName}]")
      return
    }


    atomic { implicit tx =>
      listDataKeyCheck(listName)

    // we could use CSVDictReader .. but I don't care for the column names
    // and this uses slightly less memory (on initial load)

    hasDiscriminator match {
      case true =>
        var indexKey = IndexKey(listName, ",,^^,,") // a nonsense discriminator that won't match
        for (r:Array[String] <- rows) {
          val i = listData(listName).size
          listData.put(listName, listData(listName) :+ r(0))
          if (!indexKey.discriminator.equals(r(1))) {
            indexKey = IndexKey(listName, r(1))
            indexKeyCheck(indexKey)
          }
          index.put(indexKey, index(indexKey) :+ i)
        }
      case false => for( r:Array[String] <- rows) { listData.put(listName, listData(listName) :+ r(0)) }
    }
    }
  }

  /**
   * Adding more data to an existing list
   * @param listName
   * @param data
   * @param discriminatorValue
   */
  def importDataWithDiscriminator(listName:String,data:Seq[String],discriminatorValue:String) = {

    // unlike importFile and importData, we wont check if the list has already been imported
    // we will just let it happen.

    val indexKey = IndexKey(listName,discriminatorValue) // a nonsense discriminator that won't match
    atomic { implicit tx =>
      listDataKeyCheck(listName)
      indexKeyCheck(indexKey)
    }

    data.foreach {
      v => {
        atomic { implicit tx =>
          val i = listData(listName).size
          listData.put(listName, listData(listName) :+ v)
          index.put(indexKey, index(indexKey) :+ i)
        }
      }
    }
  }

  private def indexKeyCheck(indexKey:IndexKey)(implicit txn: scala.concurrent.stm.InTxn):Unit = {
    if (! index.contains(indexKey)) {
      index.put(indexKey,Seq[Int]())
    }
  }


  private def listDataKeyCheck(listName:String)(implicit txn: scala.concurrent.stm.InTxn):Unit = {
    if (! listData.contains(listName)) {
      listData.put(listName,Seq[String]())
    }
  }

  private val csvMapper = {
    val mapper = new CsvMapper()
    mapper.enable(CsvParser.Feature.WRAP_AS_ARRAY)
    mapper
  }

  /**
   * Import a File Stream
   *
   * @param listName
   * @param stream
   */
  def importFile(listName:String, stream:InputStream):Unit = {

    if (listData.single.containsKey(listName)) {
      logger.info(s"Not importing [${listName}]")
      return
    }

    // we could use CSVDictReader .. but I don't care for the column names
    // and this uses slightly less memory (on initial load)
    val mapper = csvMapper
    val iter:MappingIterator[Array[String]] = mapper.reader(classOf[Array[String]]).readValues(stream)

    // first row is always considered the header row.. MUST have column names in
    // the CSV files people
    // look at the first row and see if has more than one column (the header row)
    val firstRow = iter.next()

    if (firstRow.length > 1) {
      importData(listName, iter, hasDiscriminator = true)
    } else {
      importData(listName, iter, hasDiscriminator = false)
    }

  }


  /**
   * Return a randomvalue, optionally using a disciminator
   * It is presumed that the list has been loaded at this point
   *
   * @param listName
   * @param discriminator
   * @return
   */
  def getRandomValue(listName:String,discriminator:Option[String] = None) = {

    discriminator match {
      case None => RandomUtil.randomFromList(listData.single(listName))
      case Some(discrim) => RandomUtil.randomFromListByIndex(listData.single(listName),index.single(IndexKey(listName,discrim)))
    }
  }

}


