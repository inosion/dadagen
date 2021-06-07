package org.inosion.dadagen.auto

case class MyTest(firstname: String, surname: String)

import org.junit.runner.RunWith
import org.scalatestplus.junit.JUnitRunner
import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.matchers.should.Matchers
@RunWith(classOf[JUnitRunner])
class ScalaClassReadTest extends AnyFlatSpec with Matchers {

  "The dadagen reader" should "populate all simple fields" in {
      val testInstances: IndexedSeq[MyTest] = dadagen[MyTest].generate().take(400).toIndexedSeq
      testInstances.size should be (400)
  }

}
