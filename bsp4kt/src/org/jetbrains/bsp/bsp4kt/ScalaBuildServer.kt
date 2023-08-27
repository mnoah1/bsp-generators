package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName
import java.util.concurrent.CompletableFuture

interface ScalaBuildServer {
  @JsonRequest("buildTarget/scalacOptions")
  fun buildTargetScalacOptions(params: ScalacOptionsParams): CompletableFuture<ScalacOptionsResult>

  @Deprecated("Use buildTarget/jvmTestEnvironment instead")
  @JsonRequest("buildTarget/scalaTestClasses")
  fun buildTargetScalaTestClasses(params: ScalaTestClassesParams): CompletableFuture<ScalaTestClassesResult>

  @Deprecated("Use buildTarget/jvmRunEnvironment instead")
  @JsonRequest("buildTarget/scalaMainClasses")
  fun buildTargetScalaMainClasses(params: ScalaMainClassesParams): CompletableFuture<ScalaMainClassesResult>

}