package org.inosion.dadagen

import org.inosion.dadagen.generators.{DependantListGenerator, GenericListGenerator, TemplateGenerator}
import org.junit.runner.RunWith
import org.scalatestplus.junit.JUnitRunner
import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.matchers.should.Matchers
@RunWith(classOf[JUnitRunner])
class DependantListTest extends AnyFlatSpec with Matchers {
    "A dependent list " should "select from the correct map" in {

      val values = Map ("a123" -> List("x","y","z"),"a556" -> List("ccc","ddd","aaa"),"a445" -> List("11x","22y","33z"))

      val ramondom = DadagenStringMap(List(
        GenericListGenerator(name = "foo", listData = Some(List("a123","a445","a556"))),
        DependantListGenerator(name = "foo_size",keyName = "foo", dependantList = values)
      ))

      val results = ramondom.generateAll(10)

      for (row <- results) {
        // foo_size should be a value from the value in the "map / list"
        values.get(row("foo")).get should contain (row("foo_size"))
      }
    }
}

