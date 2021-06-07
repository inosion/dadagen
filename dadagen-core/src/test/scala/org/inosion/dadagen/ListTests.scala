package org.inosion.dadagen

import org.inosion.dadagen.lists.ListManager
import ListManager.IndexKey
import org.inosion.dadagen.generators.{GenderGenerator, TitleGenerator}
import org.junit.runner.RunWith
import org.scalatestplus.junit.JUnitRunner
import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.matchers.should.Matchers

/**
 * @author rbuckland
 */

import scala.collection.JavaConverters._

@RunWith(classOf[JUnitRunner])
class RandomListTest extends AnyFlatSpec with Matchers {
  "Providing an array of strings" should "return a random entry from that list in the ListManager" in {

    val values = Array("blue","red","green","orange","purple","black","brown","pink")

    ListManager.importData("colours",values.map{x => List(x) }.toStream,false)

    val aSingleValue = ListManager.getRandomValue("colours",None)

    values should contain (aSingleValue)

  }
}

class MultiDataListImportTest extends  AnyFlatSpec with Matchers {
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


