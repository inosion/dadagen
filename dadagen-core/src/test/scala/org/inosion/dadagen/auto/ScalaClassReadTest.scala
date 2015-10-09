package org.inosion.dadagen.auto

import org.scalatest.{FlatSpec, Matchers}


case class MyTest(firstname: String, surname: String)

class ScalaClassReadTest extends FlatSpec with Matchers {

  "The dadagen reader" should "populate all simple fields" in {
      val testInstances: IndexedSeq[MyTest] = dadagen[MyTest].generate().take(400).toIndexedSeq
      testInstances.size should be (400)
  }

}
