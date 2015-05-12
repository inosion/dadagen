package org.inosion

import java.security.SecureRandom
import java.util.Random

/**
 * Default Random Implementation
 */
package object dadagen {

  // using secure random because we never know where dadagen will be used
  // make sure it produces non-deterministic random data as best is available

  implicit val rand: Random = new SecureRandom

}
