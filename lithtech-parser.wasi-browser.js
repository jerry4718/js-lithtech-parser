import {
  createOnMessage as __wasmCreateOnMessageForFsProxy,
  getDefaultContext as __emnapiGetDefaultContext,
  instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync,
  WASI as __WASI,
} from '@napi-rs/wasm-runtime'



const __wasi = new __WASI({
  version: 'preview1',
})

const __wasmUrl = new URL('./lithtech-parser.wasm32-wasi.wasm', import.meta.url).href
const __emnapiContext = __emnapiGetDefaultContext()


const __sharedMemory = new WebAssembly.Memory({
  initial: 4000,
  maximum: 65536,
  shared: true,
})

const __wasmFile = await fetch(__wasmUrl).then((res) => res.arrayBuffer())

const {
  instance: __napiInstance,
  module: __wasiModule,
  napiModule: __napiModule,
} = __emnapiInstantiateNapiModuleSync(__wasmFile, {
  context: __emnapiContext,
  asyncWorkPoolSize: 4,
  wasi: __wasi,
  onCreateWorker() {
    const worker = new Worker(new URL('./wasi-worker-browser.mjs', import.meta.url), {
      type: 'module',
    })

    return worker
  },
  overwriteImports(importObject) {
    importObject.env = {
      ...importObject.env,
      ...importObject.napi,
      ...importObject.emnapi,
      memory: __sharedMemory,
    }
    return importObject
  },
  beforeInit({ instance }) {
    for (const name of Object.keys(instance.exports)) {
      if (name.startsWith('__napi_register__')) {
        instance.exports[name]()
      }
    }
  },
})
export default __napiModule.exports
export const Animation = __napiModule.exports.Animation
export const AnimBinding = __napiModule.exports.AnimBinding
export const BlindData = __napiModule.exports.BlindData
export const BoneNode = __napiModule.exports.BoneNode
export const BoneSet = __napiModule.exports.BoneSet
export const ColorArgb = __napiModule.exports.ColorArgb
export const ColorBgra = __napiModule.exports.ColorBgra
export const ColorRgb = __napiModule.exports.ColorRgb
export const ColorRgba = __napiModule.exports.ColorRgba
export const Colour32bit = __napiModule.exports.Colour32bit
export const Colour8bit = __napiModule.exports.Colour8bit
export const Compressed = __napiModule.exports.Compressed
export const CompressedTextureRow = __napiModule.exports.CompressedTextureRow
export const DataItem = __napiModule.exports.DataItem
export const DatHeader = __napiModule.exports.DatHeader
export const DatVertex = __napiModule.exports.DatVertex
export const DtxHeader = __napiModule.exports.DtxHeader
export const DtxSection = __napiModule.exports.DtxSection
export const ImageMeta = __napiModule.exports.ImageMeta
export const ItemProperty = __napiModule.exports.ItemProperty
export const ItemPropertyData = __napiModule.exports.ItemPropertyData
export const Keyframe = __napiModule.exports.Keyframe
export const LightData = __napiModule.exports.LightData
export const LightGroup = __napiModule.exports.LightGroup
export const LightMapSection = __napiModule.exports.LightMapSection
export const LightMapSectionRow = __napiModule.exports.LightMapSectionRow
export const LithtechDat = __napiModule.exports.LithtechDat
export const LithtechDtx = __napiModule.exports.LithtechDtx
export const LithtechLtb = __napiModule.exports.LithtechLtb
export const LodContainer = __napiModule.exports.LodContainer
export const LodMesh = __napiModule.exports.LodMesh
export const LodMeshInfo = __napiModule.exports.LodMeshInfo
export const LtbHeader = __napiModule.exports.LtbHeader
export const LtbVertex = __napiModule.exports.LtbVertex
export const Matrix = __napiModule.exports.Matrix
export const Mipmap = __napiModule.exports.Mipmap
export const ModelOBB = __napiModule.exports.ModelOBB
export const NormalTransform = __napiModule.exports.NormalTransform
export const NullMesh = __napiModule.exports.NullMesh
export const Palette32Bit = __napiModule.exports.Palette32Bit
export const Palette8Bit = __napiModule.exports.Palette8Bit
export const Piece = __napiModule.exports.Piece
export const Plane = __napiModule.exports.Plane
export const Polygon = __napiModule.exports.Polygon
export const PolygonData = __napiModule.exports.PolygonData
export const QuaternionF32 = __napiModule.exports.QuaternionF32
export const RenderBlock = __napiModule.exports.RenderBlock
export const RenderData = __napiModule.exports.RenderData
export const RenderSection = __napiModule.exports.RenderSection
export const RigidMesh = __napiModule.exports.RigidMesh
export const ShaderPoly = __napiModule.exports.ShaderPoly
export const SkeletalMesh = __napiModule.exports.SkeletalMesh
export const SkyPortal = __napiModule.exports.SkyPortal
export const Socket = __napiModule.exports.Socket
export const Surface = __napiModule.exports.Surface
export const Texture32Bit = __napiModule.exports.Texture32Bit
export const Texture32BitTextureRow = __napiModule.exports.Texture32BitTextureRow
export const Triangle = __napiModule.exports.Triangle
export const UnknownMesh = __napiModule.exports.UnknownMesh
export const Vector2F32 = __napiModule.exports.Vector2F32
export const Vector3F32 = __napiModule.exports.Vector3F32
export const VertexAnimatedMesh = __napiModule.exports.VertexAnimatedMesh
export const VertexGroup = __napiModule.exports.VertexGroup
export const WeightSet = __napiModule.exports.WeightSet
export const World = __napiModule.exports.World
export const WorldData = __napiModule.exports.WorldData
export const WorldModel = __napiModule.exports.WorldModel
export const WorldModelNode = __napiModule.exports.WorldModelNode
export const WorldModelPolygon = __napiModule.exports.WorldModelPolygon
export const WorldModelRenderBlock = __napiModule.exports.WorldModelRenderBlock
export const WorldTree = __napiModule.exports.WorldTree
export const parseDat = __napiModule.exports.parseDat
export const parseDtx = __napiModule.exports.parseDtx
export const parseLtb = __napiModule.exports.parseLtb
export const plus100 = __napiModule.exports.plus100
export const plus100FromZig = __napiModule.exports.plus100FromZig
