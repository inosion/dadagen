package org.inosion.dadagen

import org.junit.runner.RunWith
import org.scalatestplus.junit.JUnitRunner
import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.matchers.should.Matchers

@RunWith(classOf[JUnitRunner])
class NullListCheckTest extends AnyFlatSpec with Matchers {
  
  "A null Generator List" should "throw a require error" in {
    try {
      val gen = DadagenStringMap(null)
      fail("did not error when null list provided")
    } catch {
      case _: NullPointerException => // expected
    }
  }
}

