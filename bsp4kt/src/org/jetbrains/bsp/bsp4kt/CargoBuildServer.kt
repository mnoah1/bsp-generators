package org.jetbrains.bsp.bsp4kt

import org.jetbrains.jsonrpc4kt.services.JsonNotification
import org.jetbrains.jsonrpc4kt.services.JsonRequest
import kotlinx.serialization.SerialName
import java.util.concurrent.CompletableFuture

interface CargoBuildServer {
  @JsonRequest("workspace/cargoFeaturesState")
  fun cargoFeaturesState(): CompletableFuture<CargoFeaturesStateResult>

  @JsonRequest("workspace/enableCargoFeatures")
  fun enableCargoFeatures(params: EnableCargoFeaturesParams): CompletableFuture<Unit>

  @JsonRequest("workspace/disableCargoFeatures")
  fun disableCargoFeatures(params: DisableCargoFeaturesParams): CompletableFuture<Unit>

}