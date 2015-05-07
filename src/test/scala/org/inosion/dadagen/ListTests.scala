package org.inosion.dadagen

import org.inosion.dadagen.ListManager.IndexKey
import org.inosion.dadagen.randomtypes.{GenderGenerator, TitleGenerator}
import org.scalatest.{Matchers, FlatSpec}

/**
 * @author rbuckland
 */

import scala.collection.JavaConversions._

class RandomListTest extends FlatSpec with Matchers {
  "Providing an array of strings" should "return a random entry from that list in the ListManager" in {

    val values = Array("blue","red","green","orange","purple","black","brown","pink")

    ListManager.importData("colours",values.map{x => Array(x) }.toIterator,false)

    val aSingleValue = ListManager.getRandomValue("colours",None)

    values should contain (aSingleValue)

  }
}

class MultiDataListImportTest extends  FlatSpec with Matchers {
  "Importing a list with a discriminator" should "add the data into the same named list" in {

    val femaleTitles = List("Mrs","Ms","Miss","Dr","Prof","Rev")
    val maleTitles = List("Mr","Dr","Prof","Rev")

    ListManager.importDataWithDiscriminator("titles",femaleTitles,"F")
    ListManager.importDataWithDiscriminator("titles",maleTitles,"M")

    // brute inspection test

    ListManager.listData.single("titles").size should be (femaleTitles.size + maleTitles.size)
    ListManager.index.single(IndexKey("titles","F")).size should be (femaleTitles.size)
    ListManager.index.single(IndexKey("titles","M")).size should be (maleTitles.size)
    // random tests are not that great :-/ but hey

    for (i <- 0 until 5000)
    {
      femaleTitles should contain(ListManager.getRandomValue("titles", Some("F")))
      maleTitles should contain(ListManager.getRandomValue("titles", Some("M")))
    }
  }
}


