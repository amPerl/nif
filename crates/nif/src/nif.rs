/*
 * Enum {
 *     name: "ApplyMode",
 *     storage: "uint",
 *     description: Some(
 *         "Describes how the vertex colors are blended with the filtered texture color.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "APPLY_REPLACE",
 *             description: Some(
 *                 "Replaces existing color",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "APPLY_DECAL",
 *             description: Some(
 *                 "For placing images on the object like stickers.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "APPLY_MODULATE",
 *             description: Some(
 *                 "Modulates existing color. (Default)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "APPLY_HILIGHT",
 *             description: Some(
 *                 "PS2 Only.  Function Unknown.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "APPLY_HILIGHT2",
 *             description: Some(
 *                 "Parallax Flag in some Oblivion meshes.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes how the vertex colors are blended with the filtered texture color.
#[binrw::binrw]
pub enum ApplyMode {
    #[brw(magic = 0_u32)]
    ApplyReplace,
    #[brw(magic = 1_u32)]
    ApplyDecal,
    #[brw(magic = 2_u32)]
    ApplyModulate,
    #[brw(magic = 3_u32)]
    ApplyHilight,
    #[brw(magic = 4_u32)]
    ApplyHilight2,
}
/*
 * Enum {
 *     name: "TexType",
 *     storage: "uint",
 *     description: Some(
 *         "The type of texture.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "BASE_MAP",
 *             description: Some(
 *                 "The basic texture used by most meshes.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "DARK_MAP",
 *             description: Some(
 *                 "Used to darken the model with false lighting.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "DETAIL_MAP",
 *             description: Some(
 *                 "Combined with base map for added detail.  Usually tiled over the mesh many times for close-up view.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "GLOSS_MAP",
 *             description: Some(
 *                 "Allows the specularity (glossyness) of an object to differ across its surface.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "GLOW_MAP",
 *             description: Some(
 *                 "Creates a glowing effect.  Basically an incandescence map.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "BUMP_MAP",
 *             description: Some(
 *                 "Used to make the object appear to have more detail than it really does.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "NORMAL_MAP",
 *             description: Some(
 *                 "Used to make the object appear to have more detail than it really does.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "PARALLAX_MAP",
 *             description: Some(
 *                 "Parallax map.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "DECAL_0_MAP",
 *             description: Some(
 *                 "For placing images on the object like stickers.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "DECAL_1_MAP",
 *             description: Some(
 *                 "For placing images on the object like stickers.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "DECAL_2_MAP",
 *             description: Some(
 *                 "For placing images on the object like stickers.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "11",
 *             name: "DECAL_3_MAP",
 *             description: Some(
 *                 "For placing images on the object like stickers.",
 *             ),
 *         },
 *     ],
 * }
 */
/// The type of texture.
#[binrw::binrw]
pub enum TexType {
    #[brw(magic = 0_u32)]
    BaseMap,
    #[brw(magic = 1_u32)]
    DarkMap,
    #[brw(magic = 2_u32)]
    DetailMap,
    #[brw(magic = 3_u32)]
    GlossMap,
    #[brw(magic = 4_u32)]
    GlowMap,
    #[brw(magic = 5_u32)]
    BumpMap,
    #[brw(magic = 6_u32)]
    NormalMap,
    #[brw(magic = 7_u32)]
    ParallaxMap,
    #[brw(magic = 8_u32)]
    Decal0Map,
    #[brw(magic = 9_u32)]
    Decal1Map,
    #[brw(magic = 10_u32)]
    Decal2Map,
    #[brw(magic = 11_u32)]
    Decal3Map,
}
/*
 * Enum {
 *     name: "KeyType",
 *     storage: "uint",
 *     description: Some(
 *         "The type of animation interpolation (blending) that will be used on the associated key frames.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "1",
 *             name: "LINEAR_KEY",
 *             description: Some(
 *                 "Use linear interpolation.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "QUADRATIC_KEY",
 *             description: Some(
 *                 "Use quadratic interpolation.  Forward and back tangents will be stored.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "TBC_KEY",
 *             description: Some(
 *                 "Use Tension Bias Continuity interpolation.  Tension, bias, and continuity will be stored.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "XYZ_ROTATION_KEY",
 *             description: Some(
 *                 "For use only with rotation data.  Separate X, Y, and Z keys will be stored instead of using quaternions.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "CONST_KEY",
 *             description: Some(
 *                 "Step function. Used for visibility keys in NiBoolData.",
 *             ),
 *         },
 *     ],
 * }
 */
/// The type of animation interpolation (blending) that will be used on the associated key frames.
#[binrw::binrw]
pub enum KeyType {
    #[brw(magic = 1_u32)]
    LinearKey,
    #[brw(magic = 2_u32)]
    QuadraticKey,
    #[brw(magic = 3_u32)]
    TbcKey,
    #[brw(magic = 4_u32)]
    XyzRotationKey,
    #[brw(magic = 5_u32)]
    ConstKey,
}
/*
 * Enum {
 *     name: "OblivionHavokMaterial",
 *     storage: "uint",
 *     description: Some(
 *         "Bethesda Havok. Material descriptor for a Havok shape in Oblivion.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "OB_HAV_MAT_STONE",
 *             description: Some(
 *                 "Stone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "OB_HAV_MAT_CLOTH",
 *             description: Some(
 *                 "Cloth",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "OB_HAV_MAT_DIRT",
 *             description: Some(
 *                 "Dirt",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "OB_HAV_MAT_GLASS",
 *             description: Some(
 *                 "Glass",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "OB_HAV_MAT_GRASS",
 *             description: Some(
 *                 "Grass",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "OB_HAV_MAT_METAL",
 *             description: Some(
 *                 "Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "OB_HAV_MAT_ORGANIC",
 *             description: Some(
 *                 "Organic",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "OB_HAV_MAT_SKIN",
 *             description: Some(
 *                 "Skin",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "OB_HAV_MAT_WATER",
 *             description: Some(
 *                 "Water",
 *             ),
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "OB_HAV_MAT_WOOD",
 *             description: Some(
 *                 "Wood",
 *             ),
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "OB_HAV_MAT_HEAVY_STONE",
 *             description: Some(
 *                 "Heavy Stone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "11",
 *             name: "OB_HAV_MAT_HEAVY_METAL",
 *             description: Some(
 *                 "Heavy Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "12",
 *             name: "OB_HAV_MAT_HEAVY_WOOD",
 *             description: Some(
 *                 "Heavy Wood",
 *             ),
 *         },
 *         EnumOption {
 *             value: "13",
 *             name: "OB_HAV_MAT_CHAIN",
 *             description: Some(
 *                 "Chain",
 *             ),
 *         },
 *         EnumOption {
 *             value: "14",
 *             name: "OB_HAV_MAT_SNOW",
 *             description: Some(
 *                 "Snow",
 *             ),
 *         },
 *         EnumOption {
 *             value: "15",
 *             name: "OB_HAV_MAT_STONE_STAIRS",
 *             description: Some(
 *                 "Stone Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "16",
 *             name: "OB_HAV_MAT_CLOTH_STAIRS",
 *             description: Some(
 *                 "Cloth Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "17",
 *             name: "OB_HAV_MAT_DIRT_STAIRS",
 *             description: Some(
 *                 "Dirt Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "18",
 *             name: "OB_HAV_MAT_GLASS_STAIRS",
 *             description: Some(
 *                 "Glass Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "19",
 *             name: "OB_HAV_MAT_GRASS_STAIRS",
 *             description: Some(
 *                 "Grass Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "20",
 *             name: "OB_HAV_MAT_METAL_STAIRS",
 *             description: Some(
 *                 "Metal Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "21",
 *             name: "OB_HAV_MAT_ORGANIC_STAIRS",
 *             description: Some(
 *                 "Organic Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "22",
 *             name: "OB_HAV_MAT_SKIN_STAIRS",
 *             description: Some(
 *                 "Skin Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "23",
 *             name: "OB_HAV_MAT_WATER_STAIRS",
 *             description: Some(
 *                 "Water Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "24",
 *             name: "OB_HAV_MAT_WOOD_STAIRS",
 *             description: Some(
 *                 "Wood Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "25",
 *             name: "OB_HAV_MAT_HEAVY_STONE_STAIRS",
 *             description: Some(
 *                 "Heavy Stone Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "26",
 *             name: "OB_HAV_MAT_HEAVY_METAL_STAIRS",
 *             description: Some(
 *                 "Heavy Metal Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "27",
 *             name: "OB_HAV_MAT_HEAVY_WOOD_STAIRS",
 *             description: Some(
 *                 "Heavy Wood Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "28",
 *             name: "OB_HAV_MAT_CHAIN_STAIRS",
 *             description: Some(
 *                 "Chain Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "29",
 *             name: "OB_HAV_MAT_SNOW_STAIRS",
 *             description: Some(
 *                 "Snow Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "30",
 *             name: "OB_HAV_MAT_ELEVATOR",
 *             description: Some(
 *                 "Elevator",
 *             ),
 *         },
 *         EnumOption {
 *             value: "31",
 *             name: "OB_HAV_MAT_RUBBER",
 *             description: Some(
 *                 "Rubber",
 *             ),
 *         },
 *     ],
 * }
 */
/// Bethesda Havok. Material descriptor for a Havok shape in Oblivion.
#[binrw::binrw]
pub enum OblivionHavokMaterial {
    #[brw(magic = 0_u32)]
    ObHavMatStone,
    #[brw(magic = 1_u32)]
    ObHavMatCloth,
    #[brw(magic = 2_u32)]
    ObHavMatDirt,
    #[brw(magic = 3_u32)]
    ObHavMatGlass,
    #[brw(magic = 4_u32)]
    ObHavMatGrass,
    #[brw(magic = 5_u32)]
    ObHavMatMetal,
    #[brw(magic = 6_u32)]
    ObHavMatOrganic,
    #[brw(magic = 7_u32)]
    ObHavMatSkin,
    #[brw(magic = 8_u32)]
    ObHavMatWater,
    #[brw(magic = 9_u32)]
    ObHavMatWood,
    #[brw(magic = 10_u32)]
    ObHavMatHeavyStone,
    #[brw(magic = 11_u32)]
    ObHavMatHeavyMetal,
    #[brw(magic = 12_u32)]
    ObHavMatHeavyWood,
    #[brw(magic = 13_u32)]
    ObHavMatChain,
    #[brw(magic = 14_u32)]
    ObHavMatSnow,
    #[brw(magic = 15_u32)]
    ObHavMatStoneStairs,
    #[brw(magic = 16_u32)]
    ObHavMatClothStairs,
    #[brw(magic = 17_u32)]
    ObHavMatDirtStairs,
    #[brw(magic = 18_u32)]
    ObHavMatGlassStairs,
    #[brw(magic = 19_u32)]
    ObHavMatGrassStairs,
    #[brw(magic = 20_u32)]
    ObHavMatMetalStairs,
    #[brw(magic = 21_u32)]
    ObHavMatOrganicStairs,
    #[brw(magic = 22_u32)]
    ObHavMatSkinStairs,
    #[brw(magic = 23_u32)]
    ObHavMatWaterStairs,
    #[brw(magic = 24_u32)]
    ObHavMatWoodStairs,
    #[brw(magic = 25_u32)]
    ObHavMatHeavyStoneStairs,
    #[brw(magic = 26_u32)]
    ObHavMatHeavyMetalStairs,
    #[brw(magic = 27_u32)]
    ObHavMatHeavyWoodStairs,
    #[brw(magic = 28_u32)]
    ObHavMatChainStairs,
    #[brw(magic = 29_u32)]
    ObHavMatSnowStairs,
    #[brw(magic = 30_u32)]
    ObHavMatElevator,
    #[brw(magic = 31_u32)]
    ObHavMatRubber,
}
/*
 * Enum {
 *     name: "Fallout3HavokMaterial",
 *     storage: "uint",
 *     description: Some(
 *         "Bethesda Havok. Material descriptor for a Havok shape in Fallout 3 and Fallout NV.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "FO_HAV_MAT_STONE",
 *             description: Some(
 *                 "Stone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "FO_HAV_MAT_CLOTH",
 *             description: Some(
 *                 "Cloth",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "FO_HAV_MAT_DIRT",
 *             description: Some(
 *                 "Dirt",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "FO_HAV_MAT_GLASS",
 *             description: Some(
 *                 "Glass",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "FO_HAV_MAT_GRASS",
 *             description: Some(
 *                 "Grass",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "FO_HAV_MAT_METAL",
 *             description: Some(
 *                 "Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "FO_HAV_MAT_ORGANIC",
 *             description: Some(
 *                 "Organic",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "FO_HAV_MAT_SKIN",
 *             description: Some(
 *                 "Skin",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "FO_HAV_MAT_WATER",
 *             description: Some(
 *                 "Water",
 *             ),
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "FO_HAV_MAT_WOOD",
 *             description: Some(
 *                 "Wood",
 *             ),
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "FO_HAV_MAT_HEAVY_STONE",
 *             description: Some(
 *                 "Heavy Stone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "11",
 *             name: "FO_HAV_MAT_HEAVY_METAL",
 *             description: Some(
 *                 "Heavy Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "12",
 *             name: "FO_HAV_MAT_HEAVY_WOOD",
 *             description: Some(
 *                 "Heavy Wood",
 *             ),
 *         },
 *         EnumOption {
 *             value: "13",
 *             name: "FO_HAV_MAT_CHAIN",
 *             description: Some(
 *                 "Chain",
 *             ),
 *         },
 *         EnumOption {
 *             value: "14",
 *             name: "FO_HAV_MAT_BOTTLECAP",
 *             description: Some(
 *                 "Bottlecap",
 *             ),
 *         },
 *         EnumOption {
 *             value: "15",
 *             name: "FO_HAV_MAT_ELEVATOR",
 *             description: Some(
 *                 "Elevator",
 *             ),
 *         },
 *         EnumOption {
 *             value: "16",
 *             name: "FO_HAV_MAT_HOLLOW_METAL",
 *             description: Some(
 *                 "Hollow Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "17",
 *             name: "FO_HAV_MAT_SHEET_METAL",
 *             description: Some(
 *                 "Sheet Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "18",
 *             name: "FO_HAV_MAT_SAND",
 *             description: Some(
 *                 "Sand",
 *             ),
 *         },
 *         EnumOption {
 *             value: "19",
 *             name: "FO_HAV_MAT_BROKEN_CONCRETE",
 *             description: Some(
 *                 "Broken Concrete",
 *             ),
 *         },
 *         EnumOption {
 *             value: "20",
 *             name: "FO_HAV_MAT_VEHICLE_BODY",
 *             description: Some(
 *                 "Vehicle Body",
 *             ),
 *         },
 *         EnumOption {
 *             value: "21",
 *             name: "FO_HAV_MAT_VEHICLE_PART_SOLID",
 *             description: Some(
 *                 "Vehicle Part Solid",
 *             ),
 *         },
 *         EnumOption {
 *             value: "22",
 *             name: "FO_HAV_MAT_VEHICLE_PART_HOLLOW",
 *             description: Some(
 *                 "Vehicle Part Hollow",
 *             ),
 *         },
 *         EnumOption {
 *             value: "23",
 *             name: "FO_HAV_MAT_BARREL",
 *             description: Some(
 *                 "Barrel",
 *             ),
 *         },
 *         EnumOption {
 *             value: "24",
 *             name: "FO_HAV_MAT_BOTTLE",
 *             description: Some(
 *                 "Bottle",
 *             ),
 *         },
 *         EnumOption {
 *             value: "25",
 *             name: "FO_HAV_MAT_SODA_CAN",
 *             description: Some(
 *                 "Soda Can",
 *             ),
 *         },
 *         EnumOption {
 *             value: "26",
 *             name: "FO_HAV_MAT_PISTOL",
 *             description: Some(
 *                 "Pistol",
 *             ),
 *         },
 *         EnumOption {
 *             value: "27",
 *             name: "FO_HAV_MAT_RIFLE",
 *             description: Some(
 *                 "Rifle",
 *             ),
 *         },
 *         EnumOption {
 *             value: "28",
 *             name: "FO_HAV_MAT_SHOPPING_CART",
 *             description: Some(
 *                 "Shopping Cart",
 *             ),
 *         },
 *         EnumOption {
 *             value: "29",
 *             name: "FO_HAV_MAT_LUNCHBOX",
 *             description: Some(
 *                 "Lunchbox",
 *             ),
 *         },
 *         EnumOption {
 *             value: "30",
 *             name: "FO_HAV_MAT_BABY_RATTLE",
 *             description: Some(
 *                 "Baby Rattle",
 *             ),
 *         },
 *         EnumOption {
 *             value: "31",
 *             name: "FO_HAV_MAT_RUBBER_BALL",
 *             description: Some(
 *                 "Rubber Ball",
 *             ),
 *         },
 *         EnumOption {
 *             value: "32",
 *             name: "FO_HAV_MAT_STONE_PLATFORM",
 *             description: Some(
 *                 "Stone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "33",
 *             name: "FO_HAV_MAT_CLOTH_PLATFORM",
 *             description: Some(
 *                 "Cloth",
 *             ),
 *         },
 *         EnumOption {
 *             value: "34",
 *             name: "FO_HAV_MAT_DIRT_PLATFORM",
 *             description: Some(
 *                 "Dirt",
 *             ),
 *         },
 *         EnumOption {
 *             value: "35",
 *             name: "FO_HAV_MAT_GLASS_PLATFORM",
 *             description: Some(
 *                 "Glass",
 *             ),
 *         },
 *         EnumOption {
 *             value: "36",
 *             name: "FO_HAV_MAT_GRASS_PLATFORM",
 *             description: Some(
 *                 "Grass",
 *             ),
 *         },
 *         EnumOption {
 *             value: "37",
 *             name: "FO_HAV_MAT_METAL_PLATFORM",
 *             description: Some(
 *                 "Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "38",
 *             name: "FO_HAV_MAT_ORGANIC_PLATFORM",
 *             description: Some(
 *                 "Organic",
 *             ),
 *         },
 *         EnumOption {
 *             value: "39",
 *             name: "FO_HAV_MAT_SKIN_PLATFORM",
 *             description: Some(
 *                 "Skin",
 *             ),
 *         },
 *         EnumOption {
 *             value: "40",
 *             name: "FO_HAV_MAT_WATER_PLATFORM",
 *             description: Some(
 *                 "Water",
 *             ),
 *         },
 *         EnumOption {
 *             value: "41",
 *             name: "FO_HAV_MAT_WOOD_PLATFORM",
 *             description: Some(
 *                 "Wood",
 *             ),
 *         },
 *         EnumOption {
 *             value: "42",
 *             name: "FO_HAV_MAT_HEAVY_STONE_PLATFORM",
 *             description: Some(
 *                 "Heavy Stone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "43",
 *             name: "FO_HAV_MAT_HEAVY_METAL_PLATFORM",
 *             description: Some(
 *                 "Heavy Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "44",
 *             name: "FO_HAV_MAT_HEAVY_WOOD_PLATFORM",
 *             description: Some(
 *                 "Heavy Wood",
 *             ),
 *         },
 *         EnumOption {
 *             value: "45",
 *             name: "FO_HAV_MAT_CHAIN_PLATFORM",
 *             description: Some(
 *                 "Chain",
 *             ),
 *         },
 *         EnumOption {
 *             value: "46",
 *             name: "FO_HAV_MAT_BOTTLECAP_PLATFORM",
 *             description: Some(
 *                 "Bottlecap",
 *             ),
 *         },
 *         EnumOption {
 *             value: "47",
 *             name: "FO_HAV_MAT_ELEVATOR_PLATFORM",
 *             description: Some(
 *                 "Elevator",
 *             ),
 *         },
 *         EnumOption {
 *             value: "48",
 *             name: "FO_HAV_MAT_HOLLOW_METAL_PLATFORM",
 *             description: Some(
 *                 "Hollow Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "49",
 *             name: "FO_HAV_MAT_SHEET_METAL_PLATFORM",
 *             description: Some(
 *                 "Sheet Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "50",
 *             name: "FO_HAV_MAT_SAND_PLATFORM",
 *             description: Some(
 *                 "Sand",
 *             ),
 *         },
 *         EnumOption {
 *             value: "51",
 *             name: "FO_HAV_MAT_BROKEN_CONCRETE_PLATFORM",
 *             description: Some(
 *                 "Broken Concrete",
 *             ),
 *         },
 *         EnumOption {
 *             value: "52",
 *             name: "FO_HAV_MAT_VEHICLE_BODY_PLATFORM",
 *             description: Some(
 *                 "Vehicle Body",
 *             ),
 *         },
 *         EnumOption {
 *             value: "53",
 *             name: "FO_HAV_MAT_VEHICLE_PART_SOLID_PLATFORM",
 *             description: Some(
 *                 "Vehicle Part Solid",
 *             ),
 *         },
 *         EnumOption {
 *             value: "54",
 *             name: "FO_HAV_MAT_VEHICLE_PART_HOLLOW_PLATFORM",
 *             description: Some(
 *                 "Vehicle Part Hollow",
 *             ),
 *         },
 *         EnumOption {
 *             value: "55",
 *             name: "FO_HAV_MAT_BARREL_PLATFORM",
 *             description: Some(
 *                 "Barrel",
 *             ),
 *         },
 *         EnumOption {
 *             value: "56",
 *             name: "FO_HAV_MAT_BOTTLE_PLATFORM",
 *             description: Some(
 *                 "Bottle",
 *             ),
 *         },
 *         EnumOption {
 *             value: "57",
 *             name: "FO_HAV_MAT_SODA_CAN_PLATFORM",
 *             description: Some(
 *                 "Soda Can",
 *             ),
 *         },
 *         EnumOption {
 *             value: "58",
 *             name: "FO_HAV_MAT_PISTOL_PLATFORM",
 *             description: Some(
 *                 "Pistol",
 *             ),
 *         },
 *         EnumOption {
 *             value: "59",
 *             name: "FO_HAV_MAT_RIFLE_PLATFORM",
 *             description: Some(
 *                 "Rifle",
 *             ),
 *         },
 *         EnumOption {
 *             value: "60",
 *             name: "FO_HAV_MAT_SHOPPING_CART_PLATFORM",
 *             description: Some(
 *                 "Shopping Cart",
 *             ),
 *         },
 *         EnumOption {
 *             value: "61",
 *             name: "FO_HAV_MAT_LUNCHBOX_PLATFORM",
 *             description: Some(
 *                 "Lunchbox",
 *             ),
 *         },
 *         EnumOption {
 *             value: "62",
 *             name: "FO_HAV_MAT_BABY_RATTLE_PLATFORM",
 *             description: Some(
 *                 "Baby Rattle",
 *             ),
 *         },
 *         EnumOption {
 *             value: "63",
 *             name: "FO_HAV_MAT_RUBBER_BALL_PLATFORM",
 *             description: Some(
 *                 "Rubber Ball",
 *             ),
 *         },
 *         EnumOption {
 *             value: "64",
 *             name: "FO_HAV_MAT_STONE_STAIRS",
 *             description: Some(
 *                 "Stone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "65",
 *             name: "FO_HAV_MAT_CLOTH_STAIRS",
 *             description: Some(
 *                 "Cloth",
 *             ),
 *         },
 *         EnumOption {
 *             value: "66",
 *             name: "FO_HAV_MAT_DIRT_STAIRS",
 *             description: Some(
 *                 "Dirt",
 *             ),
 *         },
 *         EnumOption {
 *             value: "67",
 *             name: "FO_HAV_MAT_GLASS_STAIRS",
 *             description: Some(
 *                 "Glass",
 *             ),
 *         },
 *         EnumOption {
 *             value: "68",
 *             name: "FO_HAV_MAT_GRASS_STAIRS",
 *             description: Some(
 *                 "Grass",
 *             ),
 *         },
 *         EnumOption {
 *             value: "69",
 *             name: "FO_HAV_MAT_METAL_STAIRS",
 *             description: Some(
 *                 "Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "70",
 *             name: "FO_HAV_MAT_ORGANIC_STAIRS",
 *             description: Some(
 *                 "Organic",
 *             ),
 *         },
 *         EnumOption {
 *             value: "71",
 *             name: "FO_HAV_MAT_SKIN_STAIRS",
 *             description: Some(
 *                 "Skin",
 *             ),
 *         },
 *         EnumOption {
 *             value: "72",
 *             name: "FO_HAV_MAT_WATER_STAIRS",
 *             description: Some(
 *                 "Water",
 *             ),
 *         },
 *         EnumOption {
 *             value: "73",
 *             name: "FO_HAV_MAT_WOOD_STAIRS",
 *             description: Some(
 *                 "Wood",
 *             ),
 *         },
 *         EnumOption {
 *             value: "74",
 *             name: "FO_HAV_MAT_HEAVY_STONE_STAIRS",
 *             description: Some(
 *                 "Heavy Stone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "75",
 *             name: "FO_HAV_MAT_HEAVY_METAL_STAIRS",
 *             description: Some(
 *                 "Heavy Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "76",
 *             name: "FO_HAV_MAT_HEAVY_WOOD_STAIRS",
 *             description: Some(
 *                 "Heavy Wood",
 *             ),
 *         },
 *         EnumOption {
 *             value: "77",
 *             name: "FO_HAV_MAT_CHAIN_STAIRS",
 *             description: Some(
 *                 "Chain",
 *             ),
 *         },
 *         EnumOption {
 *             value: "78",
 *             name: "FO_HAV_MAT_BOTTLECAP_STAIRS",
 *             description: Some(
 *                 "Bottlecap",
 *             ),
 *         },
 *         EnumOption {
 *             value: "79",
 *             name: "FO_HAV_MAT_ELEVATOR_STAIRS",
 *             description: Some(
 *                 "Elevator",
 *             ),
 *         },
 *         EnumOption {
 *             value: "80",
 *             name: "FO_HAV_MAT_HOLLOW_METAL_STAIRS",
 *             description: Some(
 *                 "Hollow Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "81",
 *             name: "FO_HAV_MAT_SHEET_METAL_STAIRS",
 *             description: Some(
 *                 "Sheet Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "82",
 *             name: "FO_HAV_MAT_SAND_STAIRS",
 *             description: Some(
 *                 "Sand",
 *             ),
 *         },
 *         EnumOption {
 *             value: "83",
 *             name: "FO_HAV_MAT_BROKEN_CONCRETE_STAIRS",
 *             description: Some(
 *                 "Broken Concrete",
 *             ),
 *         },
 *         EnumOption {
 *             value: "84",
 *             name: "FO_HAV_MAT_VEHICLE_BODY_STAIRS",
 *             description: Some(
 *                 "Vehicle Body",
 *             ),
 *         },
 *         EnumOption {
 *             value: "85",
 *             name: "FO_HAV_MAT_VEHICLE_PART_SOLID_STAIRS",
 *             description: Some(
 *                 "Vehicle Part Solid",
 *             ),
 *         },
 *         EnumOption {
 *             value: "86",
 *             name: "FO_HAV_MAT_VEHICLE_PART_HOLLOW_STAIRS",
 *             description: Some(
 *                 "Vehicle Part Hollow",
 *             ),
 *         },
 *         EnumOption {
 *             value: "87",
 *             name: "FO_HAV_MAT_BARREL_STAIRS",
 *             description: Some(
 *                 "Barrel",
 *             ),
 *         },
 *         EnumOption {
 *             value: "88",
 *             name: "FO_HAV_MAT_BOTTLE_STAIRS",
 *             description: Some(
 *                 "Bottle",
 *             ),
 *         },
 *         EnumOption {
 *             value: "89",
 *             name: "FO_HAV_MAT_SODA_CAN_STAIRS",
 *             description: Some(
 *                 "Soda Can",
 *             ),
 *         },
 *         EnumOption {
 *             value: "90",
 *             name: "FO_HAV_MAT_PISTOL_STAIRS",
 *             description: Some(
 *                 "Pistol",
 *             ),
 *         },
 *         EnumOption {
 *             value: "91",
 *             name: "FO_HAV_MAT_RIFLE_STAIRS",
 *             description: Some(
 *                 "Rifle",
 *             ),
 *         },
 *         EnumOption {
 *             value: "92",
 *             name: "FO_HAV_MAT_SHOPPING_CART_STAIRS",
 *             description: Some(
 *                 "Shopping Cart",
 *             ),
 *         },
 *         EnumOption {
 *             value: "93",
 *             name: "FO_HAV_MAT_LUNCHBOX_STAIRS",
 *             description: Some(
 *                 "Lunchbox",
 *             ),
 *         },
 *         EnumOption {
 *             value: "94",
 *             name: "FO_HAV_MAT_BABY_RATTLE_STAIRS",
 *             description: Some(
 *                 "Baby Rattle",
 *             ),
 *         },
 *         EnumOption {
 *             value: "95",
 *             name: "FO_HAV_MAT_RUBBER_BALL_STAIRS",
 *             description: Some(
 *                 "Rubber Ball",
 *             ),
 *         },
 *         EnumOption {
 *             value: "96",
 *             name: "FO_HAV_MAT_STONE_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Stone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "97",
 *             name: "FO_HAV_MAT_CLOTH_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Cloth",
 *             ),
 *         },
 *         EnumOption {
 *             value: "98",
 *             name: "FO_HAV_MAT_DIRT_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Dirt",
 *             ),
 *         },
 *         EnumOption {
 *             value: "99",
 *             name: "FO_HAV_MAT_GLASS_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Glass",
 *             ),
 *         },
 *         EnumOption {
 *             value: "100",
 *             name: "FO_HAV_MAT_GRASS_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Grass",
 *             ),
 *         },
 *         EnumOption {
 *             value: "101",
 *             name: "FO_HAV_MAT_METAL_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "102",
 *             name: "FO_HAV_MAT_ORGANIC_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Organic",
 *             ),
 *         },
 *         EnumOption {
 *             value: "103",
 *             name: "FO_HAV_MAT_SKIN_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Skin",
 *             ),
 *         },
 *         EnumOption {
 *             value: "104",
 *             name: "FO_HAV_MAT_WATER_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Water",
 *             ),
 *         },
 *         EnumOption {
 *             value: "105",
 *             name: "FO_HAV_MAT_WOOD_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Wood",
 *             ),
 *         },
 *         EnumOption {
 *             value: "106",
 *             name: "FO_HAV_MAT_HEAVY_STONE_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Heavy Stone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "107",
 *             name: "FO_HAV_MAT_HEAVY_METAL_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Heavy Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "108",
 *             name: "FO_HAV_MAT_HEAVY_WOOD_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Heavy Wood",
 *             ),
 *         },
 *         EnumOption {
 *             value: "109",
 *             name: "FO_HAV_MAT_CHAIN_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Chain",
 *             ),
 *         },
 *         EnumOption {
 *             value: "110",
 *             name: "FO_HAV_MAT_BOTTLECAP_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Bottlecap",
 *             ),
 *         },
 *         EnumOption {
 *             value: "111",
 *             name: "FO_HAV_MAT_ELEVATOR_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Elevator",
 *             ),
 *         },
 *         EnumOption {
 *             value: "112",
 *             name: "FO_HAV_MAT_HOLLOW_METAL_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Hollow Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "113",
 *             name: "FO_HAV_MAT_SHEET_METAL_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Sheet Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "114",
 *             name: "FO_HAV_MAT_SAND_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Sand",
 *             ),
 *         },
 *         EnumOption {
 *             value: "115",
 *             name: "FO_HAV_MAT_BROKEN_CONCRETE_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Broken Concrete",
 *             ),
 *         },
 *         EnumOption {
 *             value: "116",
 *             name: "FO_HAV_MAT_VEHICLE_BODY_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Vehicle Body",
 *             ),
 *         },
 *         EnumOption {
 *             value: "117",
 *             name: "FO_HAV_MAT_VEHICLE_PART_SOLID_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Vehicle Part Solid",
 *             ),
 *         },
 *         EnumOption {
 *             value: "118",
 *             name: "FO_HAV_MAT_VEHICLE_PART_HOLLOW_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Vehicle Part Hollow",
 *             ),
 *         },
 *         EnumOption {
 *             value: "119",
 *             name: "FO_HAV_MAT_BARREL_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Barrel",
 *             ),
 *         },
 *         EnumOption {
 *             value: "120",
 *             name: "FO_HAV_MAT_BOTTLE_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Bottle",
 *             ),
 *         },
 *         EnumOption {
 *             value: "121",
 *             name: "FO_HAV_MAT_SODA_CAN_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Soda Can",
 *             ),
 *         },
 *         EnumOption {
 *             value: "122",
 *             name: "FO_HAV_MAT_PISTOL_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Pistol",
 *             ),
 *         },
 *         EnumOption {
 *             value: "123",
 *             name: "FO_HAV_MAT_RIFLE_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Rifle",
 *             ),
 *         },
 *         EnumOption {
 *             value: "124",
 *             name: "FO_HAV_MAT_SHOPPING_CART_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Shopping Cart",
 *             ),
 *         },
 *         EnumOption {
 *             value: "125",
 *             name: "FO_HAV_MAT_LUNCHBOX_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Lunchbox",
 *             ),
 *         },
 *         EnumOption {
 *             value: "126",
 *             name: "FO_HAV_MAT_BABY_RATTLE_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Baby Rattle",
 *             ),
 *         },
 *         EnumOption {
 *             value: "127",
 *             name: "FO_HAV_MAT_RUBBER_BALL_STAIRS_PLATFORM",
 *             description: Some(
 *                 "Rubber Ball",
 *             ),
 *         },
 *     ],
 * }
 */
/// Bethesda Havok. Material descriptor for a Havok shape in Fallout 3 and Fallout NV.
#[binrw::binrw]
pub enum Fallout3HavokMaterial {
    #[brw(magic = 0_u32)]
    FoHavMatStone,
    #[brw(magic = 1_u32)]
    FoHavMatCloth,
    #[brw(magic = 2_u32)]
    FoHavMatDirt,
    #[brw(magic = 3_u32)]
    FoHavMatGlass,
    #[brw(magic = 4_u32)]
    FoHavMatGrass,
    #[brw(magic = 5_u32)]
    FoHavMatMetal,
    #[brw(magic = 6_u32)]
    FoHavMatOrganic,
    #[brw(magic = 7_u32)]
    FoHavMatSkin,
    #[brw(magic = 8_u32)]
    FoHavMatWater,
    #[brw(magic = 9_u32)]
    FoHavMatWood,
    #[brw(magic = 10_u32)]
    FoHavMatHeavyStone,
    #[brw(magic = 11_u32)]
    FoHavMatHeavyMetal,
    #[brw(magic = 12_u32)]
    FoHavMatHeavyWood,
    #[brw(magic = 13_u32)]
    FoHavMatChain,
    #[brw(magic = 14_u32)]
    FoHavMatBottlecap,
    #[brw(magic = 15_u32)]
    FoHavMatElevator,
    #[brw(magic = 16_u32)]
    FoHavMatHollowMetal,
    #[brw(magic = 17_u32)]
    FoHavMatSheetMetal,
    #[brw(magic = 18_u32)]
    FoHavMatSand,
    #[brw(magic = 19_u32)]
    FoHavMatBrokenConcrete,
    #[brw(magic = 20_u32)]
    FoHavMatVehicleBody,
    #[brw(magic = 21_u32)]
    FoHavMatVehiclePartSolid,
    #[brw(magic = 22_u32)]
    FoHavMatVehiclePartHollow,
    #[brw(magic = 23_u32)]
    FoHavMatBarrel,
    #[brw(magic = 24_u32)]
    FoHavMatBottle,
    #[brw(magic = 25_u32)]
    FoHavMatSodaCan,
    #[brw(magic = 26_u32)]
    FoHavMatPistol,
    #[brw(magic = 27_u32)]
    FoHavMatRifle,
    #[brw(magic = 28_u32)]
    FoHavMatShoppingCart,
    #[brw(magic = 29_u32)]
    FoHavMatLunchbox,
    #[brw(magic = 30_u32)]
    FoHavMatBabyRattle,
    #[brw(magic = 31_u32)]
    FoHavMatRubberBall,
    #[brw(magic = 32_u32)]
    FoHavMatStonePlatform,
    #[brw(magic = 33_u32)]
    FoHavMatClothPlatform,
    #[brw(magic = 34_u32)]
    FoHavMatDirtPlatform,
    #[brw(magic = 35_u32)]
    FoHavMatGlassPlatform,
    #[brw(magic = 36_u32)]
    FoHavMatGrassPlatform,
    #[brw(magic = 37_u32)]
    FoHavMatMetalPlatform,
    #[brw(magic = 38_u32)]
    FoHavMatOrganicPlatform,
    #[brw(magic = 39_u32)]
    FoHavMatSkinPlatform,
    #[brw(magic = 40_u32)]
    FoHavMatWaterPlatform,
    #[brw(magic = 41_u32)]
    FoHavMatWoodPlatform,
    #[brw(magic = 42_u32)]
    FoHavMatHeavyStonePlatform,
    #[brw(magic = 43_u32)]
    FoHavMatHeavyMetalPlatform,
    #[brw(magic = 44_u32)]
    FoHavMatHeavyWoodPlatform,
    #[brw(magic = 45_u32)]
    FoHavMatChainPlatform,
    #[brw(magic = 46_u32)]
    FoHavMatBottlecapPlatform,
    #[brw(magic = 47_u32)]
    FoHavMatElevatorPlatform,
    #[brw(magic = 48_u32)]
    FoHavMatHollowMetalPlatform,
    #[brw(magic = 49_u32)]
    FoHavMatSheetMetalPlatform,
    #[brw(magic = 50_u32)]
    FoHavMatSandPlatform,
    #[brw(magic = 51_u32)]
    FoHavMatBrokenConcretePlatform,
    #[brw(magic = 52_u32)]
    FoHavMatVehicleBodyPlatform,
    #[brw(magic = 53_u32)]
    FoHavMatVehiclePartSolidPlatform,
    #[brw(magic = 54_u32)]
    FoHavMatVehiclePartHollowPlatform,
    #[brw(magic = 55_u32)]
    FoHavMatBarrelPlatform,
    #[brw(magic = 56_u32)]
    FoHavMatBottlePlatform,
    #[brw(magic = 57_u32)]
    FoHavMatSodaCanPlatform,
    #[brw(magic = 58_u32)]
    FoHavMatPistolPlatform,
    #[brw(magic = 59_u32)]
    FoHavMatRiflePlatform,
    #[brw(magic = 60_u32)]
    FoHavMatShoppingCartPlatform,
    #[brw(magic = 61_u32)]
    FoHavMatLunchboxPlatform,
    #[brw(magic = 62_u32)]
    FoHavMatBabyRattlePlatform,
    #[brw(magic = 63_u32)]
    FoHavMatRubberBallPlatform,
    #[brw(magic = 64_u32)]
    FoHavMatStoneStairs,
    #[brw(magic = 65_u32)]
    FoHavMatClothStairs,
    #[brw(magic = 66_u32)]
    FoHavMatDirtStairs,
    #[brw(magic = 67_u32)]
    FoHavMatGlassStairs,
    #[brw(magic = 68_u32)]
    FoHavMatGrassStairs,
    #[brw(magic = 69_u32)]
    FoHavMatMetalStairs,
    #[brw(magic = 70_u32)]
    FoHavMatOrganicStairs,
    #[brw(magic = 71_u32)]
    FoHavMatSkinStairs,
    #[brw(magic = 72_u32)]
    FoHavMatWaterStairs,
    #[brw(magic = 73_u32)]
    FoHavMatWoodStairs,
    #[brw(magic = 74_u32)]
    FoHavMatHeavyStoneStairs,
    #[brw(magic = 75_u32)]
    FoHavMatHeavyMetalStairs,
    #[brw(magic = 76_u32)]
    FoHavMatHeavyWoodStairs,
    #[brw(magic = 77_u32)]
    FoHavMatChainStairs,
    #[brw(magic = 78_u32)]
    FoHavMatBottlecapStairs,
    #[brw(magic = 79_u32)]
    FoHavMatElevatorStairs,
    #[brw(magic = 80_u32)]
    FoHavMatHollowMetalStairs,
    #[brw(magic = 81_u32)]
    FoHavMatSheetMetalStairs,
    #[brw(magic = 82_u32)]
    FoHavMatSandStairs,
    #[brw(magic = 83_u32)]
    FoHavMatBrokenConcreteStairs,
    #[brw(magic = 84_u32)]
    FoHavMatVehicleBodyStairs,
    #[brw(magic = 85_u32)]
    FoHavMatVehiclePartSolidStairs,
    #[brw(magic = 86_u32)]
    FoHavMatVehiclePartHollowStairs,
    #[brw(magic = 87_u32)]
    FoHavMatBarrelStairs,
    #[brw(magic = 88_u32)]
    FoHavMatBottleStairs,
    #[brw(magic = 89_u32)]
    FoHavMatSodaCanStairs,
    #[brw(magic = 90_u32)]
    FoHavMatPistolStairs,
    #[brw(magic = 91_u32)]
    FoHavMatRifleStairs,
    #[brw(magic = 92_u32)]
    FoHavMatShoppingCartStairs,
    #[brw(magic = 93_u32)]
    FoHavMatLunchboxStairs,
    #[brw(magic = 94_u32)]
    FoHavMatBabyRattleStairs,
    #[brw(magic = 95_u32)]
    FoHavMatRubberBallStairs,
    #[brw(magic = 96_u32)]
    FoHavMatStoneStairsPlatform,
    #[brw(magic = 97_u32)]
    FoHavMatClothStairsPlatform,
    #[brw(magic = 98_u32)]
    FoHavMatDirtStairsPlatform,
    #[brw(magic = 99_u32)]
    FoHavMatGlassStairsPlatform,
    #[brw(magic = 100_u32)]
    FoHavMatGrassStairsPlatform,
    #[brw(magic = 101_u32)]
    FoHavMatMetalStairsPlatform,
    #[brw(magic = 102_u32)]
    FoHavMatOrganicStairsPlatform,
    #[brw(magic = 103_u32)]
    FoHavMatSkinStairsPlatform,
    #[brw(magic = 104_u32)]
    FoHavMatWaterStairsPlatform,
    #[brw(magic = 105_u32)]
    FoHavMatWoodStairsPlatform,
    #[brw(magic = 106_u32)]
    FoHavMatHeavyStoneStairsPlatform,
    #[brw(magic = 107_u32)]
    FoHavMatHeavyMetalStairsPlatform,
    #[brw(magic = 108_u32)]
    FoHavMatHeavyWoodStairsPlatform,
    #[brw(magic = 109_u32)]
    FoHavMatChainStairsPlatform,
    #[brw(magic = 110_u32)]
    FoHavMatBottlecapStairsPlatform,
    #[brw(magic = 111_u32)]
    FoHavMatElevatorStairsPlatform,
    #[brw(magic = 112_u32)]
    FoHavMatHollowMetalStairsPlatform,
    #[brw(magic = 113_u32)]
    FoHavMatSheetMetalStairsPlatform,
    #[brw(magic = 114_u32)]
    FoHavMatSandStairsPlatform,
    #[brw(magic = 115_u32)]
    FoHavMatBrokenConcreteStairsPlatform,
    #[brw(magic = 116_u32)]
    FoHavMatVehicleBodyStairsPlatform,
    #[brw(magic = 117_u32)]
    FoHavMatVehiclePartSolidStairsPlatform,
    #[brw(magic = 118_u32)]
    FoHavMatVehiclePartHollowStairsPlatform,
    #[brw(magic = 119_u32)]
    FoHavMatBarrelStairsPlatform,
    #[brw(magic = 120_u32)]
    FoHavMatBottleStairsPlatform,
    #[brw(magic = 121_u32)]
    FoHavMatSodaCanStairsPlatform,
    #[brw(magic = 122_u32)]
    FoHavMatPistolStairsPlatform,
    #[brw(magic = 123_u32)]
    FoHavMatRifleStairsPlatform,
    #[brw(magic = 124_u32)]
    FoHavMatShoppingCartStairsPlatform,
    #[brw(magic = 125_u32)]
    FoHavMatLunchboxStairsPlatform,
    #[brw(magic = 126_u32)]
    FoHavMatBabyRattleStairsPlatform,
    #[brw(magic = 127_u32)]
    FoHavMatRubberBallStairsPlatform,
}
/*
 * Enum {
 *     name: "SkyrimHavokMaterial",
 *     storage: "uint",
 *     description: Some(
 *         "Bethesda Havok. Material descriptor for a Havok shape in Skyrim. CRC32 of the lowercase of the Creation Kit Material Name.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "SKY_HAV_MAT_NONE",
 *             description: Some(
 *                 "Invalid Material",
 *             ),
 *         },
 *         EnumOption {
 *             value: "131151687",
 *             name: "SKY_HAV_MAT_BROKEN_STONE",
 *             description: Some(
 *                 "Broken Stone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "322207473",
 *             name: "SKY_HAV_MAT_MATERIAL_CARRIAGE_WHEEL",
 *             description: Some(
 *                 "Material Carriage Wheel",
 *             ),
 *         },
 *         EnumOption {
 *             value: "346811165",
 *             name: "SKY_HAV_MAT_MATERIAL_METAL_LIGHT",
 *             description: Some(
 *                 "Material Metal Light",
 *             ),
 *         },
 *         EnumOption {
 *             value: "365420259",
 *             name: "SKY_HAV_MAT_LIGHT_WOOD",
 *             description: Some(
 *                 "Light Wood",
 *             ),
 *         },
 *         EnumOption {
 *             value: "398949039",
 *             name: "SKY_HAV_MAT_SNOW",
 *             description: Some(
 *                 "Snow",
 *             ),
 *         },
 *         EnumOption {
 *             value: "428587608",
 *             name: "SKY_HAV_MAT_GRAVEL",
 *             description: Some(
 *                 "Gravel",
 *             ),
 *         },
 *         EnumOption {
 *             value: "438912228",
 *             name: "SKY_HAV_MAT_MATERIAL_CHAIN_METAL",
 *             description: Some(
 *                 "Material Chain Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "493553910",
 *             name: "SKY_HAV_MAT_BOTTLE",
 *             description: Some(
 *                 "Bottle",
 *             ),
 *         },
 *         EnumOption {
 *             value: "500811281",
 *             name: "SKY_HAV_MAT_WOOD",
 *             description: Some(
 *                 "Wood",
 *             ),
 *         },
 *         EnumOption {
 *             value: "591247106",
 *             name: "SKY_HAV_MAT_SKIN",
 *             description: Some(
 *                 "Skin",
 *             ),
 *         },
 *         EnumOption {
 *             value: "617099282",
 *             name: "SKY_HAV_MAT_UNKNOWN_617099282",
 *             description: Some(
 *                 "Unknown in Creation Kit v1.9.32.0. Found in Dawnguard DLC in meshes\\dlc01\\clutter\\dlc01deerskin.nif.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "732141076",
 *             name: "SKY_HAV_MAT_BARREL",
 *             description: Some(
 *                 "Barrel",
 *             ),
 *         },
 *         EnumOption {
 *             value: "781661019",
 *             name: "SKY_HAV_MAT_MATERIAL_CERAMIC_MEDIUM",
 *             description: Some(
 *                 "Material Ceramic Medium",
 *             ),
 *         },
 *         EnumOption {
 *             value: "790784366",
 *             name: "SKY_HAV_MAT_MATERIAL_BASKET",
 *             description: Some(
 *                 "Material Basket",
 *             ),
 *         },
 *         EnumOption {
 *             value: "873356572",
 *             name: "SKY_HAV_MAT_ICE",
 *             description: Some(
 *                 "Ice",
 *             ),
 *         },
 *         EnumOption {
 *             value: "880200008",
 *             name: "SKY_HAV_MAT_STAIRS_GLASS",
 *             description: Some(
 *                 "Stairs Glass",
 *             ),
 *         },
 *         EnumOption {
 *             value: "899511101",
 *             name: "SKY_HAV_MAT_STAIRS_STONE",
 *             description: Some(
 *                 "Stairs Stone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1024582599",
 *             name: "SKY_HAV_MAT_WATER",
 *             description: Some(
 *                 "Water",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1028101969",
 *             name: "SKY_HAV_MAT_UNKNOWN_1028101969",
 *             description: Some(
 *                 "Unknown in Creation Kit v1.6.89.0. Found in actors\\draugr\\character assets\\skeletons.nif.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1060167844",
 *             name: "SKY_HAV_MAT_MATERIAL_BLADE_1HAND",
 *             description: Some(
 *                 "Material Blade 1 Hand",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1264672850",
 *             name: "SKY_HAV_MAT_MATERIAL_BOOK",
 *             description: Some(
 *                 "Material Book",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1286705471",
 *             name: "SKY_HAV_MAT_MATERIAL_CARPET",
 *             description: Some(
 *                 "Material Carpet",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1288358971",
 *             name: "SKY_HAV_MAT_SOLID_METAL",
 *             description: Some(
 *                 "Solid Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1305674443",
 *             name: "SKY_HAV_MAT_MATERIAL_AXE_1HAND",
 *             description: Some(
 *                 "Material Axe 1Hand",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1440721808",
 *             name: "SKY_HAV_MAT_UNKNOWN_1440721808",
 *             description: Some(
 *                 "Unknown in Creation Kit v1.6.89.0. Found in armor\\draugr\\draugrbootsfemale_go.nif or armor\\amuletsandrings\\amuletgnd.nif.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1461712277",
 *             name: "SKY_HAV_MAT_STAIRS_WOOD",
 *             description: Some(
 *                 "Stairs Wood",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1486385281",
 *             name: "SKY_HAV_MAT_MUD",
 *             description: Some(
 *                 "Mud",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1550912982",
 *             name: "SKY_HAV_MAT_MATERIAL_BOULDER_SMALL",
 *             description: Some(
 *                 "Material Boulder Small",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1560365355",
 *             name: "SKY_HAV_MAT_STAIRS_SNOW",
 *             description: Some(
 *                 "Stairs Snow",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1570821952",
 *             name: "SKY_HAV_MAT_HEAVY_STONE",
 *             description: Some(
 *                 "Heavy Stone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1574477864",
 *             name: "SKY_HAV_MAT_UNKNOWN_1574477864",
 *             description: Some(
 *                 "Unknown in Creation Kit v1.6.89.0. Found in actors\\dragon\\character assets\\skeleton.nif.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1591009235",
 *             name: "SKY_HAV_MAT_UNKNOWN_1591009235",
 *             description: Some(
 *                 "Unknown in Creation Kit v1.6.89.0. Found in trap objects or clutter\\displaycases\\displaycaselgangled01.nif or actors\\deer\\character assets\\skeleton.nif.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1607128641",
 *             name: "SKY_HAV_MAT_MATERIAL_BOWS_STAVES",
 *             description: Some(
 *                 "Material Bows Staves",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1803571212",
 *             name: "SKY_HAV_MAT_MATERIAL_WOOD_AS_STAIRS",
 *             description: Some(
 *                 "Material Wood As Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1848600814",
 *             name: "SKY_HAV_MAT_GRASS",
 *             description: Some(
 *                 "Grass",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1885326971",
 *             name: "SKY_HAV_MAT_MATERIAL_BOULDER_LARGE",
 *             description: Some(
 *                 "Material Boulder Large",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1886078335",
 *             name: "SKY_HAV_MAT_MATERIAL_STONE_AS_STAIRS",
 *             description: Some(
 *                 "Material Stone As Stairs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2022742644",
 *             name: "SKY_HAV_MAT_MATERIAL_BLADE_2HAND",
 *             description: Some(
 *                 "Material Blade 2Hand",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2025794648",
 *             name: "SKY_HAV_MAT_MATERIAL_BOTTLE_SMALL",
 *             description: Some(
 *                 "Material Bottle Small",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2168343821",
 *             name: "SKY_HAV_MAT_SAND",
 *             description: Some(
 *                 "Sand",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2229413539",
 *             name: "SKY_HAV_MAT_HEAVY_METAL",
 *             description: Some(
 *                 "Heavy Metal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2290050264",
 *             name: "SKY_HAV_MAT_UNKNOWN_2290050264",
 *             description: Some(
 *                 "Unknown in Creation Kit v1.9.32.0. Found in Dawnguard DLC in meshes\\dlc01\\clutter\\dlc01sabrecatpelt.nif.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2518321175",
 *             name: "SKY_HAV_MAT_DRAGON",
 *             description: Some(
 *                 "Dragon",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2617944780",
 *             name: "SKY_HAV_MAT_MATERIAL_BLADE_1HAND_SMALL",
 *             description: Some(
 *                 "Material Blade 1Hand Small",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2632367422",
 *             name: "SKY_HAV_MAT_MATERIAL_SKIN_SMALL",
 *             description: Some(
 *                 "Material Skin Small",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2742858142",
 *             name: "SKY_HAV_MAT_MATERIAL_POTS_PANS",
 *             description: Some(
 *                 "Material Pots Pans",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2892392795",
 *             name: "SKY_HAV_MAT_STAIRS_BROKEN_STONE",
 *             description: Some(
 *                 "Stairs Broken Stone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2965929619",
 *             name: "SKY_HAV_MAT_MATERIAL_SKIN_LARGE",
 *             description: Some(
 *                 "Material Skin Large",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2974920155",
 *             name: "SKY_HAV_MAT_ORGANIC",
 *             description: Some(
 *                 "Organic",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3049421844",
 *             name: "SKY_HAV_MAT_MATERIAL_BONE",
 *             description: Some(
 *                 "Material Bone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3070783559",
 *             name: "SKY_HAV_MAT_HEAVY_WOOD",
 *             description: Some(
 *                 "Heavy Wood",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3074114406",
 *             name: "SKY_HAV_MAT_MATERIAL_CHAIN",
 *             description: Some(
 *                 "Material Chain",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3106094762",
 *             name: "SKY_HAV_MAT_DIRT",
 *             description: Some(
 *                 "Dirt",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3387452107",
 *             name: "SKY_HAV_MAT_MATERIAL_SKIN_METAL_LARGE",
 *             description: Some(
 *                 "Material Skin Metal Large",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3424720541",
 *             name: "SKY_HAV_MAT_MATERIAL_ARMOR_LIGHT",
 *             description: Some(
 *                 "Material Armor Light",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3448167928",
 *             name: "SKY_HAV_MAT_MATERIAL_SHIELD_LIGHT",
 *             description: Some(
 *                 "Material Shield Light",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3589100606",
 *             name: "SKY_HAV_MAT_MATERIAL_COIN",
 *             description: Some(
 *                 "Material Coin",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3702389584",
 *             name: "SKY_HAV_MAT_MATERIAL_SHIELD_HEAVY",
 *             description: Some(
 *                 "Material Shield Heavy",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3708432437",
 *             name: "SKY_HAV_MAT_MATERIAL_ARMOR_HEAVY",
 *             description: Some(
 *                 "Material Armor Heavy",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3725505938",
 *             name: "SKY_HAV_MAT_MATERIAL_ARROW",
 *             description: Some(
 *                 "Material Arrow",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3739830338",
 *             name: "SKY_HAV_MAT_GLASS",
 *             description: Some(
 *                 "Glass",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3741512247",
 *             name: "SKY_HAV_MAT_STONE",
 *             description: Some(
 *                 "Stone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3764646153",
 *             name: "SKY_HAV_MAT_MATERIAL_WATER_PUDDLE",
 *             description: Some(
 *                 "Material Water Puddle",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3839073443",
 *             name: "SKY_HAV_MAT_CLOTH",
 *             description: Some(
 *                 "Cloth",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3855001958",
 *             name: "SKY_HAV_MAT_MATERIAL_SKIN_METAL_SMALL",
 *             description: Some(
 *                 "Material Skin Metal Small",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3895166727",
 *             name: "SKY_HAV_MAT_WARD",
 *             description: Some(
 *                 "Ward",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3934839107",
 *             name: "SKY_HAV_MAT_WEB",
 *             description: Some(
 *                 "Web",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3969592277",
 *             name: "SKY_HAV_MAT_MATERIAL_BLUNT_2HAND",
 *             description: Some(
 *                 "Material Blunt 2Hand",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4239621792",
 *             name: "SKY_HAV_MAT_UNKNOWN_4239621792",
 *             description: Some(
 *                 "Unknown in Creation Kit v1.9.32.0. Found in Dawnguard DLC in meshes\\dlc01\\prototype\\dlc1protoswingingbridge.nif.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4283869410",
 *             name: "SKY_HAV_MAT_MATERIAL_BOULDER_MEDIUM",
 *             description: Some(
 *                 "Material Boulder Medium",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2794252627",
 *             name: "SKY_HAV_MAT_UNKNOWN_2794252627",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1668849266",
 *             name: "SKY_HAV_MAT_UNKNOWN_1668849266",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1734341287",
 *             name: "SKY_HAV_MAT_UNKNOWN_1734341287",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3974071006",
 *             name: "SKY_HAV_MAT_UNKNOWN_3974071006",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3941234649",
 *             name: "SKY_HAV_MAT_UNKNOWN_3941234649",
 *             description: Some(
 *                 "tfxsteelswordbloody",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1820198263",
 *             name: "SKY_HAV_MAT_UNKNOWN_1820198263",
 *             description: Some(
 *                 "steelgreatsword",
 *             ),
 *         },
 *     ],
 * }
 */
/// Bethesda Havok. Material descriptor for a Havok shape in Skyrim. CRC32 of the lowercase of the Creation Kit Material Name.
#[binrw::binrw]
pub enum SkyrimHavokMaterial {
    #[brw(magic = 0_u32)]
    SkyHavMatNone,
    #[brw(magic = 131151687_u32)]
    SkyHavMatBrokenStone,
    #[brw(magic = 322207473_u32)]
    SkyHavMatMaterialCarriageWheel,
    #[brw(magic = 346811165_u32)]
    SkyHavMatMaterialMetalLight,
    #[brw(magic = 365420259_u32)]
    SkyHavMatLightWood,
    #[brw(magic = 398949039_u32)]
    SkyHavMatSnow,
    #[brw(magic = 428587608_u32)]
    SkyHavMatGravel,
    #[brw(magic = 438912228_u32)]
    SkyHavMatMaterialChainMetal,
    #[brw(magic = 493553910_u32)]
    SkyHavMatBottle,
    #[brw(magic = 500811281_u32)]
    SkyHavMatWood,
    #[brw(magic = 591247106_u32)]
    SkyHavMatSkin,
    #[brw(magic = 617099282_u32)]
    SkyHavMatUnknown617099282,
    #[brw(magic = 732141076_u32)]
    SkyHavMatBarrel,
    #[brw(magic = 781661019_u32)]
    SkyHavMatMaterialCeramicMedium,
    #[brw(magic = 790784366_u32)]
    SkyHavMatMaterialBasket,
    #[brw(magic = 873356572_u32)]
    SkyHavMatIce,
    #[brw(magic = 880200008_u32)]
    SkyHavMatStairsGlass,
    #[brw(magic = 899511101_u32)]
    SkyHavMatStairsStone,
    #[brw(magic = 1024582599_u32)]
    SkyHavMatWater,
    #[brw(magic = 1028101969_u32)]
    SkyHavMatUnknown1028101969,
    #[brw(magic = 1060167844_u32)]
    SkyHavMatMaterialBlade1hand,
    #[brw(magic = 1264672850_u32)]
    SkyHavMatMaterialBook,
    #[brw(magic = 1286705471_u32)]
    SkyHavMatMaterialCarpet,
    #[brw(magic = 1288358971_u32)]
    SkyHavMatSolidMetal,
    #[brw(magic = 1305674443_u32)]
    SkyHavMatMaterialAxe1hand,
    #[brw(magic = 1440721808_u32)]
    SkyHavMatUnknown1440721808,
    #[brw(magic = 1461712277_u32)]
    SkyHavMatStairsWood,
    #[brw(magic = 1486385281_u32)]
    SkyHavMatMud,
    #[brw(magic = 1550912982_u32)]
    SkyHavMatMaterialBoulderSmall,
    #[brw(magic = 1560365355_u32)]
    SkyHavMatStairsSnow,
    #[brw(magic = 1570821952_u32)]
    SkyHavMatHeavyStone,
    #[brw(magic = 1574477864_u32)]
    SkyHavMatUnknown1574477864,
    #[brw(magic = 1591009235_u32)]
    SkyHavMatUnknown1591009235,
    #[brw(magic = 1607128641_u32)]
    SkyHavMatMaterialBowsStaves,
    #[brw(magic = 1803571212_u32)]
    SkyHavMatMaterialWoodAsStairs,
    #[brw(magic = 1848600814_u32)]
    SkyHavMatGrass,
    #[brw(magic = 1885326971_u32)]
    SkyHavMatMaterialBoulderLarge,
    #[brw(magic = 1886078335_u32)]
    SkyHavMatMaterialStoneAsStairs,
    #[brw(magic = 2022742644_u32)]
    SkyHavMatMaterialBlade2hand,
    #[brw(magic = 2025794648_u32)]
    SkyHavMatMaterialBottleSmall,
    #[brw(magic = 2168343821_u32)]
    SkyHavMatSand,
    #[brw(magic = 2229413539_u32)]
    SkyHavMatHeavyMetal,
    #[brw(magic = 2290050264_u32)]
    SkyHavMatUnknown2290050264,
    #[brw(magic = 2518321175_u32)]
    SkyHavMatDragon,
    #[brw(magic = 2617944780_u32)]
    SkyHavMatMaterialBlade1handSmall,
    #[brw(magic = 2632367422_u32)]
    SkyHavMatMaterialSkinSmall,
    #[brw(magic = 2742858142_u32)]
    SkyHavMatMaterialPotsPans,
    #[brw(magic = 2892392795_u32)]
    SkyHavMatStairsBrokenStone,
    #[brw(magic = 2965929619_u32)]
    SkyHavMatMaterialSkinLarge,
    #[brw(magic = 2974920155_u32)]
    SkyHavMatOrganic,
    #[brw(magic = 3049421844_u32)]
    SkyHavMatMaterialBone,
    #[brw(magic = 3070783559_u32)]
    SkyHavMatHeavyWood,
    #[brw(magic = 3074114406_u32)]
    SkyHavMatMaterialChain,
    #[brw(magic = 3106094762_u32)]
    SkyHavMatDirt,
    #[brw(magic = 3387452107_u32)]
    SkyHavMatMaterialSkinMetalLarge,
    #[brw(magic = 3424720541_u32)]
    SkyHavMatMaterialArmorLight,
    #[brw(magic = 3448167928_u32)]
    SkyHavMatMaterialShieldLight,
    #[brw(magic = 3589100606_u32)]
    SkyHavMatMaterialCoin,
    #[brw(magic = 3702389584_u32)]
    SkyHavMatMaterialShieldHeavy,
    #[brw(magic = 3708432437_u32)]
    SkyHavMatMaterialArmorHeavy,
    #[brw(magic = 3725505938_u32)]
    SkyHavMatMaterialArrow,
    #[brw(magic = 3739830338_u32)]
    SkyHavMatGlass,
    #[brw(magic = 3741512247_u32)]
    SkyHavMatStone,
    #[brw(magic = 3764646153_u32)]
    SkyHavMatMaterialWaterPuddle,
    #[brw(magic = 3839073443_u32)]
    SkyHavMatCloth,
    #[brw(magic = 3855001958_u32)]
    SkyHavMatMaterialSkinMetalSmall,
    #[brw(magic = 3895166727_u32)]
    SkyHavMatWard,
    #[brw(magic = 3934839107_u32)]
    SkyHavMatWeb,
    #[brw(magic = 3969592277_u32)]
    SkyHavMatMaterialBlunt2hand,
    #[brw(magic = 4239621792_u32)]
    SkyHavMatUnknown4239621792,
    #[brw(magic = 4283869410_u32)]
    SkyHavMatMaterialBoulderMedium,
    #[brw(magic = 2794252627_u32)]
    SkyHavMatUnknown2794252627,
    #[brw(magic = 1668849266_u32)]
    SkyHavMatUnknown1668849266,
    #[brw(magic = 1734341287_u32)]
    SkyHavMatUnknown1734341287,
    #[brw(magic = 3974071006_u32)]
    SkyHavMatUnknown3974071006,
    #[brw(magic = 3941234649_u32)]
    SkyHavMatUnknown3941234649,
    #[brw(magic = 1820198263_u32)]
    SkyHavMatUnknown1820198263,
}
/*
 * Enum {
 *     name: "OblivionLayer",
 *     storage: "byte",
 *     description: Some(
 *         "Bethesda Havok. Describes the collision layer a body belongs to in Oblivion.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "OL_UNIDENTIFIED",
 *             description: Some(
 *                 "Unidentified (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "OL_STATIC",
 *             description: Some(
 *                 "Static (red)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "OL_ANIM_STATIC",
 *             description: Some(
 *                 "AnimStatic (magenta)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "OL_TRANSPARENT",
 *             description: Some(
 *                 "Transparent (light pink)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "OL_CLUTTER",
 *             description: Some(
 *                 "Clutter (light blue)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "OL_WEAPON",
 *             description: Some(
 *                 "Weapon (orange)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "OL_PROJECTILE",
 *             description: Some(
 *                 "Projectile (light orange)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "OL_SPELL",
 *             description: Some(
 *                 "Spell (cyan)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "OL_BIPED",
 *             description: Some(
 *                 "Biped (green) Seems to apply to all creatures/NPCs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "OL_TREES",
 *             description: Some(
 *                 "Trees (light brown)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "OL_PROPS",
 *             description: Some(
 *                 "Props (magenta)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "11",
 *             name: "OL_WATER",
 *             description: Some(
 *                 "Water (cyan)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "12",
 *             name: "OL_TRIGGER",
 *             description: Some(
 *                 "Trigger (light grey)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "13",
 *             name: "OL_TERRAIN",
 *             description: Some(
 *                 "Terrain (light yellow)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "14",
 *             name: "OL_TRAP",
 *             description: Some(
 *                 "Trap (light grey)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "15",
 *             name: "OL_NONCOLLIDABLE",
 *             description: Some(
 *                 "NonCollidable (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "16",
 *             name: "OL_CLOUD_TRAP",
 *             description: Some(
 *                 "CloudTrap (greenish grey)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "17",
 *             name: "OL_GROUND",
 *             description: Some(
 *                 "Ground (none)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "18",
 *             name: "OL_PORTAL",
 *             description: Some(
 *                 "Portal (green)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "19",
 *             name: "OL_STAIRS",
 *             description: Some(
 *                 "Stairs (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "20",
 *             name: "OL_CHAR_CONTROLLER",
 *             description: Some(
 *                 "CharController (yellow)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "21",
 *             name: "OL_AVOID_BOX",
 *             description: Some(
 *                 "AvoidBox (dark yellow)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "22",
 *             name: "OL_UNKNOWN1",
 *             description: Some(
 *                 "? (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "23",
 *             name: "OL_UNKNOWN2",
 *             description: Some(
 *                 "? (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "24",
 *             name: "OL_CAMERA_PICK",
 *             description: Some(
 *                 "CameraPick (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "25",
 *             name: "OL_ITEM_PICK",
 *             description: Some(
 *                 "ItemPick (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "26",
 *             name: "OL_LINE_OF_SIGHT",
 *             description: Some(
 *                 "LineOfSight (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "27",
 *             name: "OL_PATH_PICK",
 *             description: Some(
 *                 "PathPick (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "28",
 *             name: "OL_CUSTOM_PICK_1",
 *             description: Some(
 *                 "CustomPick1 (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "29",
 *             name: "OL_CUSTOM_PICK_2",
 *             description: Some(
 *                 "CustomPick2 (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "30",
 *             name: "OL_SPELL_EXPLOSION",
 *             description: Some(
 *                 "SpellExplosion (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "31",
 *             name: "OL_DROPPING_PICK",
 *             description: Some(
 *                 "DroppingPick (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "32",
 *             name: "OL_OTHER",
 *             description: Some(
 *                 "Other (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "33",
 *             name: "OL_HEAD",
 *             description: Some(
 *                 "Head",
 *             ),
 *         },
 *         EnumOption {
 *             value: "34",
 *             name: "OL_BODY",
 *             description: Some(
 *                 "Body",
 *             ),
 *         },
 *         EnumOption {
 *             value: "35",
 *             name: "OL_SPINE1",
 *             description: Some(
 *                 "Spine1",
 *             ),
 *         },
 *         EnumOption {
 *             value: "36",
 *             name: "OL_SPINE2",
 *             description: Some(
 *                 "Spine2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "37",
 *             name: "OL_L_UPPER_ARM",
 *             description: Some(
 *                 "LUpperArm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "38",
 *             name: "OL_L_FOREARM",
 *             description: Some(
 *                 "LForeArm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "39",
 *             name: "OL_L_HAND",
 *             description: Some(
 *                 "LHand",
 *             ),
 *         },
 *         EnumOption {
 *             value: "40",
 *             name: "OL_L_THIGH",
 *             description: Some(
 *                 "LThigh",
 *             ),
 *         },
 *         EnumOption {
 *             value: "41",
 *             name: "OL_L_CALF",
 *             description: Some(
 *                 "LCalf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "42",
 *             name: "OL_L_FOOT",
 *             description: Some(
 *                 "LFoot",
 *             ),
 *         },
 *         EnumOption {
 *             value: "43",
 *             name: "OL_R_UPPER_ARM",
 *             description: Some(
 *                 "RUpperArm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "44",
 *             name: "OL_R_FOREARM",
 *             description: Some(
 *                 "RForeArm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "45",
 *             name: "OL_R_HAND",
 *             description: Some(
 *                 "RHand",
 *             ),
 *         },
 *         EnumOption {
 *             value: "46",
 *             name: "OL_R_THIGH",
 *             description: Some(
 *                 "RThigh",
 *             ),
 *         },
 *         EnumOption {
 *             value: "47",
 *             name: "OL_R_CALF",
 *             description: Some(
 *                 "RCalf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "48",
 *             name: "OL_R_FOOT",
 *             description: Some(
 *                 "RFoot",
 *             ),
 *         },
 *         EnumOption {
 *             value: "49",
 *             name: "OL_TAIL",
 *             description: Some(
 *                 "Tail",
 *             ),
 *         },
 *         EnumOption {
 *             value: "50",
 *             name: "OL_SIDE_WEAPON",
 *             description: Some(
 *                 "SideWeapon",
 *             ),
 *         },
 *         EnumOption {
 *             value: "51",
 *             name: "OL_SHIELD",
 *             description: Some(
 *                 "Shield",
 *             ),
 *         },
 *         EnumOption {
 *             value: "52",
 *             name: "OL_QUIVER",
 *             description: Some(
 *                 "Quiver",
 *             ),
 *         },
 *         EnumOption {
 *             value: "53",
 *             name: "OL_BACK_WEAPON",
 *             description: Some(
 *                 "BackWeapon",
 *             ),
 *         },
 *         EnumOption {
 *             value: "54",
 *             name: "OL_BACK_WEAPON2",
 *             description: Some(
 *                 "BackWeapon (?)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "55",
 *             name: "OL_PONYTAIL",
 *             description: Some(
 *                 "PonyTail",
 *             ),
 *         },
 *         EnumOption {
 *             value: "56",
 *             name: "OL_WING",
 *             description: Some(
 *                 "Wing",
 *             ),
 *         },
 *         EnumOption {
 *             value: "57",
 *             name: "OL_NULL",
 *             description: Some(
 *                 "Null",
 *             ),
 *         },
 *     ],
 * }
 */
/// Bethesda Havok. Describes the collision layer a body belongs to in Oblivion.
#[binrw::binrw]
pub enum OblivionLayer {
    #[brw(magic = 0_u8)]
    OlUnidentified,
    #[brw(magic = 1_u8)]
    OlStatic,
    #[brw(magic = 2_u8)]
    OlAnimStatic,
    #[brw(magic = 3_u8)]
    OlTransparent,
    #[brw(magic = 4_u8)]
    OlClutter,
    #[brw(magic = 5_u8)]
    OlWeapon,
    #[brw(magic = 6_u8)]
    OlProjectile,
    #[brw(magic = 7_u8)]
    OlSpell,
    #[brw(magic = 8_u8)]
    OlBiped,
    #[brw(magic = 9_u8)]
    OlTrees,
    #[brw(magic = 10_u8)]
    OlProps,
    #[brw(magic = 11_u8)]
    OlWater,
    #[brw(magic = 12_u8)]
    OlTrigger,
    #[brw(magic = 13_u8)]
    OlTerrain,
    #[brw(magic = 14_u8)]
    OlTrap,
    #[brw(magic = 15_u8)]
    OlNoncollidable,
    #[brw(magic = 16_u8)]
    OlCloudTrap,
    #[brw(magic = 17_u8)]
    OlGround,
    #[brw(magic = 18_u8)]
    OlPortal,
    #[brw(magic = 19_u8)]
    OlStairs,
    #[brw(magic = 20_u8)]
    OlCharController,
    #[brw(magic = 21_u8)]
    OlAvoidBox,
    #[brw(magic = 22_u8)]
    OlUnknown1,
    #[brw(magic = 23_u8)]
    OlUnknown2,
    #[brw(magic = 24_u8)]
    OlCameraPick,
    #[brw(magic = 25_u8)]
    OlItemPick,
    #[brw(magic = 26_u8)]
    OlLineOfSight,
    #[brw(magic = 27_u8)]
    OlPathPick,
    #[brw(magic = 28_u8)]
    OlCustomPick1,
    #[brw(magic = 29_u8)]
    OlCustomPick2,
    #[brw(magic = 30_u8)]
    OlSpellExplosion,
    #[brw(magic = 31_u8)]
    OlDroppingPick,
    #[brw(magic = 32_u8)]
    OlOther,
    #[brw(magic = 33_u8)]
    OlHead,
    #[brw(magic = 34_u8)]
    OlBody,
    #[brw(magic = 35_u8)]
    OlSpine1,
    #[brw(magic = 36_u8)]
    OlSpine2,
    #[brw(magic = 37_u8)]
    OlLUpperArm,
    #[brw(magic = 38_u8)]
    OlLForearm,
    #[brw(magic = 39_u8)]
    OlLHand,
    #[brw(magic = 40_u8)]
    OlLThigh,
    #[brw(magic = 41_u8)]
    OlLCalf,
    #[brw(magic = 42_u8)]
    OlLFoot,
    #[brw(magic = 43_u8)]
    OlRUpperArm,
    #[brw(magic = 44_u8)]
    OlRForearm,
    #[brw(magic = 45_u8)]
    OlRHand,
    #[brw(magic = 46_u8)]
    OlRThigh,
    #[brw(magic = 47_u8)]
    OlRCalf,
    #[brw(magic = 48_u8)]
    OlRFoot,
    #[brw(magic = 49_u8)]
    OlTail,
    #[brw(magic = 50_u8)]
    OlSideWeapon,
    #[brw(magic = 51_u8)]
    OlShield,
    #[brw(magic = 52_u8)]
    OlQuiver,
    #[brw(magic = 53_u8)]
    OlBackWeapon,
    #[brw(magic = 54_u8)]
    OlBackWeapon2,
    #[brw(magic = 55_u8)]
    OlPonytail,
    #[brw(magic = 56_u8)]
    OlWing,
    #[brw(magic = 57_u8)]
    OlNull,
}
/*
 * Enum {
 *     name: "Fallout3Layer",
 *     storage: "byte",
 *     description: Some(
 *         "Bethesda Havok. Describes the collision layer a body belongs to in Fallout 3 and Fallout NV.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "FOL_UNIDENTIFIED",
 *             description: Some(
 *                 "Unidentified (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "FOL_STATIC",
 *             description: Some(
 *                 "Static (red)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "FOL_ANIM_STATIC",
 *             description: Some(
 *                 "AnimStatic (magenta)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "FOL_TRANSPARENT",
 *             description: Some(
 *                 "Transparent (light pink)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "FOL_CLUTTER",
 *             description: Some(
 *                 "Clutter (light blue)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "FOL_WEAPON",
 *             description: Some(
 *                 "Weapon (orange)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "FOL_PROJECTILE",
 *             description: Some(
 *                 "Projectile (light orange)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "FOL_SPELL",
 *             description: Some(
 *                 "Spell (cyan)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "FOL_BIPED",
 *             description: Some(
 *                 "Biped (green) Seems to apply to all creatures/NPCs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "FOL_TREES",
 *             description: Some(
 *                 "Trees (light brown)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "FOL_PROPS",
 *             description: Some(
 *                 "Props (magenta)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "11",
 *             name: "FOL_WATER",
 *             description: Some(
 *                 "Water (cyan)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "12",
 *             name: "FOL_TRIGGER",
 *             description: Some(
 *                 "Trigger (light grey)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "13",
 *             name: "FOL_TERRAIN",
 *             description: Some(
 *                 "Terrain (light yellow)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "14",
 *             name: "FOL_TRAP",
 *             description: Some(
 *                 "Trap (light grey)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "15",
 *             name: "FOL_NONCOLLIDABLE",
 *             description: Some(
 *                 "NonCollidable (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "16",
 *             name: "FOL_CLOUD_TRAP",
 *             description: Some(
 *                 "CloudTrap (greenish grey)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "17",
 *             name: "FOL_GROUND",
 *             description: Some(
 *                 "Ground (none)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "18",
 *             name: "FOL_PORTAL",
 *             description: Some(
 *                 "Portal (green)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "19",
 *             name: "FOL_DEBRIS_SMALL",
 *             description: Some(
 *                 "DebrisSmall (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "20",
 *             name: "FOL_DEBRIS_LARGE",
 *             description: Some(
 *                 "DebrisLarge (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "21",
 *             name: "FOL_ACOUSTIC_SPACE",
 *             description: Some(
 *                 "AcousticSpace (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "22",
 *             name: "FOL_ACTORZONE",
 *             description: Some(
 *                 "Actorzone (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "23",
 *             name: "FOL_PROJECTILEZONE",
 *             description: Some(
 *                 "Projectilezone (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "24",
 *             name: "FOL_GASTRAP",
 *             description: Some(
 *                 "GasTrap (yellowish green)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "25",
 *             name: "FOL_SHELLCASING",
 *             description: Some(
 *                 "ShellCasing (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "26",
 *             name: "FOL_TRANSPARENT_SMALL",
 *             description: Some(
 *                 "TransparentSmall (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "27",
 *             name: "FOL_INVISIBLE_WALL",
 *             description: Some(
 *                 "InvisibleWall (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "28",
 *             name: "FOL_TRANSPARENT_SMALL_ANIM",
 *             description: Some(
 *                 "TransparentSmallAnim (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "29",
 *             name: "FOL_DEADBIP",
 *             description: Some(
 *                 "Dead Biped (green)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "30",
 *             name: "FOL_CHARCONTROLLER",
 *             description: Some(
 *                 "CharController (yellow)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "31",
 *             name: "FOL_AVOIDBOX",
 *             description: Some(
 *                 "Avoidbox (orange)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "32",
 *             name: "FOL_COLLISIONBOX",
 *             description: Some(
 *                 "Collisionbox (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "33",
 *             name: "FOL_CAMERASPHERE",
 *             description: Some(
 *                 "Camerasphere (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "34",
 *             name: "FOL_DOORDETECTION",
 *             description: Some(
 *                 "Doordetection (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "35",
 *             name: "FOL_CAMERAPICK",
 *             description: Some(
 *                 "Camerapick (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "36",
 *             name: "FOL_ITEMPICK",
 *             description: Some(
 *                 "Itempick (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "37",
 *             name: "FOL_LINEOFSIGHT",
 *             description: Some(
 *                 "LineOfSight (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "38",
 *             name: "FOL_PATHPICK",
 *             description: Some(
 *                 "Pathpick (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "39",
 *             name: "FOL_CUSTOMPICK1",
 *             description: Some(
 *                 "Custompick1 (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "40",
 *             name: "FOL_CUSTOMPICK2",
 *             description: Some(
 *                 "Custompick2 (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "41",
 *             name: "FOL_SPELLEXPLOSION",
 *             description: Some(
 *                 "SpellExplosion (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "42",
 *             name: "FOL_DROPPINGPICK",
 *             description: Some(
 *                 "Droppingpick (white)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "43",
 *             name: "FOL_NULL",
 *             description: Some(
 *                 "Null (white)",
 *             ),
 *         },
 *     ],
 * }
 */
/// Bethesda Havok. Describes the collision layer a body belongs to in Fallout 3 and Fallout NV.
#[binrw::binrw]
pub enum Fallout3Layer {
    #[brw(magic = 0_u8)]
    FolUnidentified,
    #[brw(magic = 1_u8)]
    FolStatic,
    #[brw(magic = 2_u8)]
    FolAnimStatic,
    #[brw(magic = 3_u8)]
    FolTransparent,
    #[brw(magic = 4_u8)]
    FolClutter,
    #[brw(magic = 5_u8)]
    FolWeapon,
    #[brw(magic = 6_u8)]
    FolProjectile,
    #[brw(magic = 7_u8)]
    FolSpell,
    #[brw(magic = 8_u8)]
    FolBiped,
    #[brw(magic = 9_u8)]
    FolTrees,
    #[brw(magic = 10_u8)]
    FolProps,
    #[brw(magic = 11_u8)]
    FolWater,
    #[brw(magic = 12_u8)]
    FolTrigger,
    #[brw(magic = 13_u8)]
    FolTerrain,
    #[brw(magic = 14_u8)]
    FolTrap,
    #[brw(magic = 15_u8)]
    FolNoncollidable,
    #[brw(magic = 16_u8)]
    FolCloudTrap,
    #[brw(magic = 17_u8)]
    FolGround,
    #[brw(magic = 18_u8)]
    FolPortal,
    #[brw(magic = 19_u8)]
    FolDebrisSmall,
    #[brw(magic = 20_u8)]
    FolDebrisLarge,
    #[brw(magic = 21_u8)]
    FolAcousticSpace,
    #[brw(magic = 22_u8)]
    FolActorzone,
    #[brw(magic = 23_u8)]
    FolProjectilezone,
    #[brw(magic = 24_u8)]
    FolGastrap,
    #[brw(magic = 25_u8)]
    FolShellcasing,
    #[brw(magic = 26_u8)]
    FolTransparentSmall,
    #[brw(magic = 27_u8)]
    FolInvisibleWall,
    #[brw(magic = 28_u8)]
    FolTransparentSmallAnim,
    #[brw(magic = 29_u8)]
    FolDeadbip,
    #[brw(magic = 30_u8)]
    FolCharcontroller,
    #[brw(magic = 31_u8)]
    FolAvoidbox,
    #[brw(magic = 32_u8)]
    FolCollisionbox,
    #[brw(magic = 33_u8)]
    FolCamerasphere,
    #[brw(magic = 34_u8)]
    FolDoordetection,
    #[brw(magic = 35_u8)]
    FolCamerapick,
    #[brw(magic = 36_u8)]
    FolItempick,
    #[brw(magic = 37_u8)]
    FolLineofsight,
    #[brw(magic = 38_u8)]
    FolPathpick,
    #[brw(magic = 39_u8)]
    FolCustompick1,
    #[brw(magic = 40_u8)]
    FolCustompick2,
    #[brw(magic = 41_u8)]
    FolSpellexplosion,
    #[brw(magic = 42_u8)]
    FolDroppingpick,
    #[brw(magic = 43_u8)]
    FolNull,
}
/*
 * Enum {
 *     name: "SkyrimLayer",
 *     storage: "byte",
 *     description: Some(
 *         "Bethesda Havok. Describes the collision layer a body belongs to in Skyrim.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "SKYL_UNIDENTIFIED",
 *             description: Some(
 *                 "Unidentified",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "SKYL_STATIC",
 *             description: Some(
 *                 "Static",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "SKYL_ANIMSTATIC",
 *             description: Some(
 *                 "Anim Static",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "SKYL_TRANSPARENT",
 *             description: Some(
 *                 "Transparent",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "SKYL_CLUTTER",
 *             description: Some(
 *                 "Clutter. Object with this layer will float on water surface.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "SKYL_WEAPON",
 *             description: Some(
 *                 "Weapon",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "SKYL_PROJECTILE",
 *             description: Some(
 *                 "Projectile",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "SKYL_SPELL",
 *             description: Some(
 *                 "Spell",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "SKYL_BIPED",
 *             description: Some(
 *                 "Biped. Seems to apply to all creatures/NPCs",
 *             ),
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "SKYL_TREES",
 *             description: Some(
 *                 "Trees",
 *             ),
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "SKYL_PROPS",
 *             description: Some(
 *                 "Props",
 *             ),
 *         },
 *         EnumOption {
 *             value: "11",
 *             name: "SKYL_WATER",
 *             description: Some(
 *                 "Water",
 *             ),
 *         },
 *         EnumOption {
 *             value: "12",
 *             name: "SKYL_TRIGGER",
 *             description: Some(
 *                 "Trigger",
 *             ),
 *         },
 *         EnumOption {
 *             value: "13",
 *             name: "SKYL_TERRAIN",
 *             description: Some(
 *                 "Terrain",
 *             ),
 *         },
 *         EnumOption {
 *             value: "14",
 *             name: "SKYL_TRAP",
 *             description: Some(
 *                 "Trap",
 *             ),
 *         },
 *         EnumOption {
 *             value: "15",
 *             name: "SKYL_NONCOLLIDABLE",
 *             description: Some(
 *                 "NonCollidable",
 *             ),
 *         },
 *         EnumOption {
 *             value: "16",
 *             name: "SKYL_CLOUD_TRAP",
 *             description: Some(
 *                 "CloudTrap",
 *             ),
 *         },
 *         EnumOption {
 *             value: "17",
 *             name: "SKYL_GROUND",
 *             description: Some(
 *                 "Ground. It seems that produces no sound when collide.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "18",
 *             name: "SKYL_PORTAL",
 *             description: Some(
 *                 "Portal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "19",
 *             name: "SKYL_DEBRIS_SMALL",
 *             description: Some(
 *                 "Debris Small",
 *             ),
 *         },
 *         EnumOption {
 *             value: "20",
 *             name: "SKYL_DEBRIS_LARGE",
 *             description: Some(
 *                 "Debris Large",
 *             ),
 *         },
 *         EnumOption {
 *             value: "21",
 *             name: "SKYL_ACOUSTIC_SPACE",
 *             description: Some(
 *                 "Acoustic Space",
 *             ),
 *         },
 *         EnumOption {
 *             value: "22",
 *             name: "SKYL_ACTORZONE",
 *             description: Some(
 *                 "Actor Zone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "23",
 *             name: "SKYL_PROJECTILEZONE",
 *             description: Some(
 *                 "Projectile Zone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "24",
 *             name: "SKYL_GASTRAP",
 *             description: Some(
 *                 "Gas Trap",
 *             ),
 *         },
 *         EnumOption {
 *             value: "25",
 *             name: "SKYL_SHELLCASING",
 *             description: Some(
 *                 "Shell Casing",
 *             ),
 *         },
 *         EnumOption {
 *             value: "26",
 *             name: "SKYL_TRANSPARENT_SMALL",
 *             description: Some(
 *                 "Transparent Small",
 *             ),
 *         },
 *         EnumOption {
 *             value: "27",
 *             name: "SKYL_INVISIBLE_WALL",
 *             description: Some(
 *                 "Invisible Wall",
 *             ),
 *         },
 *         EnumOption {
 *             value: "28",
 *             name: "SKYL_TRANSPARENT_SMALL_ANIM",
 *             description: Some(
 *                 "Transparent Small Anim",
 *             ),
 *         },
 *         EnumOption {
 *             value: "29",
 *             name: "SKYL_WARD",
 *             description: Some(
 *                 "Ward",
 *             ),
 *         },
 *         EnumOption {
 *             value: "30",
 *             name: "SKYL_CHARCONTROLLER",
 *             description: Some(
 *                 "Char Controller",
 *             ),
 *         },
 *         EnumOption {
 *             value: "31",
 *             name: "SKYL_STAIRHELPER",
 *             description: Some(
 *                 "Stair Helper",
 *             ),
 *         },
 *         EnumOption {
 *             value: "32",
 *             name: "SKYL_DEADBIP",
 *             description: Some(
 *                 "Dead Bip",
 *             ),
 *         },
 *         EnumOption {
 *             value: "33",
 *             name: "SKYL_BIPED_NO_CC",
 *             description: Some(
 *                 "Biped No CC",
 *             ),
 *         },
 *         EnumOption {
 *             value: "34",
 *             name: "SKYL_AVOIDBOX",
 *             description: Some(
 *                 "Avoid Box",
 *             ),
 *         },
 *         EnumOption {
 *             value: "35",
 *             name: "SKYL_COLLISIONBOX",
 *             description: Some(
 *                 "Collision Box",
 *             ),
 *         },
 *         EnumOption {
 *             value: "36",
 *             name: "SKYL_CAMERASHPERE",
 *             description: Some(
 *                 "Camera Sphere",
 *             ),
 *         },
 *         EnumOption {
 *             value: "37",
 *             name: "SKYL_DOORDETECTION",
 *             description: Some(
 *                 "Door Detection",
 *             ),
 *         },
 *         EnumOption {
 *             value: "38",
 *             name: "SKYL_CONEPROJECTILE",
 *             description: Some(
 *                 "Cone Projectile",
 *             ),
 *         },
 *         EnumOption {
 *             value: "39",
 *             name: "SKYL_CAMERAPICK",
 *             description: Some(
 *                 "Camera Pick",
 *             ),
 *         },
 *         EnumOption {
 *             value: "40",
 *             name: "SKYL_ITEMPICK",
 *             description: Some(
 *                 "Item Pick",
 *             ),
 *         },
 *         EnumOption {
 *             value: "41",
 *             name: "SKYL_LINEOFSIGHT",
 *             description: Some(
 *                 "Line of Sight",
 *             ),
 *         },
 *         EnumOption {
 *             value: "42",
 *             name: "SKYL_PATHPICK",
 *             description: Some(
 *                 "Path Pick",
 *             ),
 *         },
 *         EnumOption {
 *             value: "43",
 *             name: "SKYL_CUSTOMPICK1",
 *             description: Some(
 *                 "Custom Pick 1",
 *             ),
 *         },
 *         EnumOption {
 *             value: "44",
 *             name: "SKYL_CUSTOMPICK2",
 *             description: Some(
 *                 "Custom Pick 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "45",
 *             name: "SKYL_SPELLEXPLOSION",
 *             description: Some(
 *                 "Spell Explosion",
 *             ),
 *         },
 *         EnumOption {
 *             value: "46",
 *             name: "SKYL_DROPPINGPICK",
 *             description: Some(
 *                 "Dropping Pick",
 *             ),
 *         },
 *         EnumOption {
 *             value: "47",
 *             name: "SKYL_DEADACTORZONE",
 *             description: Some(
 *                 "Dead Actor Zone",
 *             ),
 *         },
 *         EnumOption {
 *             value: "48",
 *             name: "SKYL_TRIGGER_FALLINGTRAP",
 *             description: Some(
 *                 "Falling Trap Trigger",
 *             ),
 *         },
 *         EnumOption {
 *             value: "49",
 *             name: "SKYL_NAVCUT",
 *             description: Some(
 *                 "Nav Cut",
 *             ),
 *         },
 *         EnumOption {
 *             value: "50",
 *             name: "SKYL_CRITTER",
 *             description: Some(
 *                 "Critter",
 *             ),
 *         },
 *         EnumOption {
 *             value: "51",
 *             name: "SKYL_SPELLTRIGGER",
 *             description: Some(
 *                 "Spell Trigger",
 *             ),
 *         },
 *         EnumOption {
 *             value: "52",
 *             name: "SKYL_LIVING_AND_DEAD_ACTORS",
 *             description: Some(
 *                 "Living And Dead Actors",
 *             ),
 *         },
 *         EnumOption {
 *             value: "53",
 *             name: "SKYL_DETECTION",
 *             description: Some(
 *                 "Detection",
 *             ),
 *         },
 *         EnumOption {
 *             value: "54",
 *             name: "SKYL_TRAP_TRIGGER",
 *             description: Some(
 *                 "Trap Trigger",
 *             ),
 *         },
 *     ],
 * }
 */
/// Bethesda Havok. Describes the collision layer a body belongs to in Skyrim.
#[binrw::binrw]
pub enum SkyrimLayer {
    #[brw(magic = 0_u8)]
    SkylUnidentified,
    #[brw(magic = 1_u8)]
    SkylStatic,
    #[brw(magic = 2_u8)]
    SkylAnimstatic,
    #[brw(magic = 3_u8)]
    SkylTransparent,
    #[brw(magic = 4_u8)]
    SkylClutter,
    #[brw(magic = 5_u8)]
    SkylWeapon,
    #[brw(magic = 6_u8)]
    SkylProjectile,
    #[brw(magic = 7_u8)]
    SkylSpell,
    #[brw(magic = 8_u8)]
    SkylBiped,
    #[brw(magic = 9_u8)]
    SkylTrees,
    #[brw(magic = 10_u8)]
    SkylProps,
    #[brw(magic = 11_u8)]
    SkylWater,
    #[brw(magic = 12_u8)]
    SkylTrigger,
    #[brw(magic = 13_u8)]
    SkylTerrain,
    #[brw(magic = 14_u8)]
    SkylTrap,
    #[brw(magic = 15_u8)]
    SkylNoncollidable,
    #[brw(magic = 16_u8)]
    SkylCloudTrap,
    #[brw(magic = 17_u8)]
    SkylGround,
    #[brw(magic = 18_u8)]
    SkylPortal,
    #[brw(magic = 19_u8)]
    SkylDebrisSmall,
    #[brw(magic = 20_u8)]
    SkylDebrisLarge,
    #[brw(magic = 21_u8)]
    SkylAcousticSpace,
    #[brw(magic = 22_u8)]
    SkylActorzone,
    #[brw(magic = 23_u8)]
    SkylProjectilezone,
    #[brw(magic = 24_u8)]
    SkylGastrap,
    #[brw(magic = 25_u8)]
    SkylShellcasing,
    #[brw(magic = 26_u8)]
    SkylTransparentSmall,
    #[brw(magic = 27_u8)]
    SkylInvisibleWall,
    #[brw(magic = 28_u8)]
    SkylTransparentSmallAnim,
    #[brw(magic = 29_u8)]
    SkylWard,
    #[brw(magic = 30_u8)]
    SkylCharcontroller,
    #[brw(magic = 31_u8)]
    SkylStairhelper,
    #[brw(magic = 32_u8)]
    SkylDeadbip,
    #[brw(magic = 33_u8)]
    SkylBipedNoCc,
    #[brw(magic = 34_u8)]
    SkylAvoidbox,
    #[brw(magic = 35_u8)]
    SkylCollisionbox,
    #[brw(magic = 36_u8)]
    SkylCamerashpere,
    #[brw(magic = 37_u8)]
    SkylDoordetection,
    #[brw(magic = 38_u8)]
    SkylConeprojectile,
    #[brw(magic = 39_u8)]
    SkylCamerapick,
    #[brw(magic = 40_u8)]
    SkylItempick,
    #[brw(magic = 41_u8)]
    SkylLineofsight,
    #[brw(magic = 42_u8)]
    SkylPathpick,
    #[brw(magic = 43_u8)]
    SkylCustompick1,
    #[brw(magic = 44_u8)]
    SkylCustompick2,
    #[brw(magic = 45_u8)]
    SkylSpellexplosion,
    #[brw(magic = 46_u8)]
    SkylDroppingpick,
    #[brw(magic = 47_u8)]
    SkylDeadactorzone,
    #[brw(magic = 48_u8)]
    SkylTriggerFallingtrap,
    #[brw(magic = 49_u8)]
    SkylNavcut,
    #[brw(magic = 50_u8)]
    SkylCritter,
    #[brw(magic = 51_u8)]
    SkylSpelltrigger,
    #[brw(magic = 52_u8)]
    SkylLivingAndDeadActors,
    #[brw(magic = 53_u8)]
    SkylDetection,
    #[brw(magic = 54_u8)]
    SkylTrapTrigger,
}
/*
 * Enum {
 *     name: "BipedPart",
 *     storage: "byte",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "P_OTHER",
 *             description: Some(
 *                 "Other",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "P_HEAD",
 *             description: Some(
 *                 "Head",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "P_BODY",
 *             description: Some(
 *                 "Body",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "P_SPINE1",
 *             description: Some(
 *                 "Spine1",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "P_SPINE2",
 *             description: Some(
 *                 "Spine2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "P_L_UPPER_ARM",
 *             description: Some(
 *                 "LUpperArm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "P_L_FOREARM",
 *             description: Some(
 *                 "LForeArm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "P_L_HAND",
 *             description: Some(
 *                 "LHand",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "P_L_THIGH",
 *             description: Some(
 *                 "LThigh",
 *             ),
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "P_L_CALF",
 *             description: Some(
 *                 "LCalf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "P_L_FOOT",
 *             description: Some(
 *                 "LFoot",
 *             ),
 *         },
 *         EnumOption {
 *             value: "11",
 *             name: "P_R_UPPER_ARM",
 *             description: Some(
 *                 "RUpperArm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "12",
 *             name: "P_R_FOREARM",
 *             description: Some(
 *                 "RForeArm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "13",
 *             name: "P_R_HAND",
 *             description: Some(
 *                 "RHand",
 *             ),
 *         },
 *         EnumOption {
 *             value: "14",
 *             name: "P_R_THIGH",
 *             description: Some(
 *                 "RThigh",
 *             ),
 *         },
 *         EnumOption {
 *             value: "15",
 *             name: "P_R_CALF",
 *             description: Some(
 *                 "RCalf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "16",
 *             name: "P_R_FOOT",
 *             description: Some(
 *                 "RFoot",
 *             ),
 *         },
 *         EnumOption {
 *             value: "17",
 *             name: "P_TAIL",
 *             description: Some(
 *                 "Tail",
 *             ),
 *         },
 *         EnumOption {
 *             value: "18",
 *             name: "P_SHIELD",
 *             description: Some(
 *                 "Shield",
 *             ),
 *         },
 *         EnumOption {
 *             value: "19",
 *             name: "P_QUIVER",
 *             description: Some(
 *                 "Quiver",
 *             ),
 *         },
 *         EnumOption {
 *             value: "20",
 *             name: "P_WEAPON",
 *             description: Some(
 *                 "Weapon",
 *             ),
 *         },
 *         EnumOption {
 *             value: "21",
 *             name: "P_PONYTAIL",
 *             description: Some(
 *                 "Ponytail",
 *             ),
 *         },
 *         EnumOption {
 *             value: "22",
 *             name: "P_WING",
 *             description: Some(
 *                 "Wing",
 *             ),
 *         },
 *         EnumOption {
 *             value: "23",
 *             name: "P_PACK",
 *             description: Some(
 *                 "Pack",
 *             ),
 *         },
 *         EnumOption {
 *             value: "24",
 *             name: "P_CHAIN",
 *             description: Some(
 *                 "Chain",
 *             ),
 *         },
 *         EnumOption {
 *             value: "25",
 *             name: "P_ADDON_HEAD",
 *             description: Some(
 *                 "AddonHead",
 *             ),
 *         },
 *         EnumOption {
 *             value: "26",
 *             name: "P_ADDON_CHEST",
 *             description: Some(
 *                 "AddonChest",
 *             ),
 *         },
 *         EnumOption {
 *             value: "27",
 *             name: "P_ADDON_LEG",
 *             description: Some(
 *                 "AddonLeg",
 *             ),
 *         },
 *         EnumOption {
 *             value: "28",
 *             name: "P_ADDON_ARM",
 *             description: Some(
 *                 "AddonArm",
 *             ),
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum BipedPart {
    #[brw(magic = 0_u8)]
    POther,
    #[brw(magic = 1_u8)]
    PHead,
    #[brw(magic = 2_u8)]
    PBody,
    #[brw(magic = 3_u8)]
    PSpine1,
    #[brw(magic = 4_u8)]
    PSpine2,
    #[brw(magic = 5_u8)]
    PLUpperArm,
    #[brw(magic = 6_u8)]
    PLForearm,
    #[brw(magic = 7_u8)]
    PLHand,
    #[brw(magic = 8_u8)]
    PLThigh,
    #[brw(magic = 9_u8)]
    PLCalf,
    #[brw(magic = 10_u8)]
    PLFoot,
    #[brw(magic = 11_u8)]
    PRUpperArm,
    #[brw(magic = 12_u8)]
    PRForearm,
    #[brw(magic = 13_u8)]
    PRHand,
    #[brw(magic = 14_u8)]
    PRThigh,
    #[brw(magic = 15_u8)]
    PRCalf,
    #[brw(magic = 16_u8)]
    PRFoot,
    #[brw(magic = 17_u8)]
    PTail,
    #[brw(magic = 18_u8)]
    PShield,
    #[brw(magic = 19_u8)]
    PQuiver,
    #[brw(magic = 20_u8)]
    PWeapon,
    #[brw(magic = 21_u8)]
    PPonytail,
    #[brw(magic = 22_u8)]
    PWing,
    #[brw(magic = 23_u8)]
    PPack,
    #[brw(magic = 24_u8)]
    PChain,
    #[brw(magic = 25_u8)]
    PAddonHead,
    #[brw(magic = 26_u8)]
    PAddonChest,
    #[brw(magic = 27_u8)]
    PAddonLeg,
    #[brw(magic = 28_u8)]
    PAddonArm,
}
/*
 * Enum {
 *     name: "hkMoppCodeBuildType",
 *     storage: "byte",
 *     description: Some(
 *         "hkpMoppCode::BuildType\r\n        A byte describing if MOPP Data is organized into chunks (PS3) or not (PC)",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "BUILT_WITH_CHUNK_SUBDIVISION",
 *             description: Some(
 *                 "Organized in chunks for PS3.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "BUILT_WITHOUT_CHUNK_SUBDIVISION",
 *             description: Some(
 *                 "Not organized in chunks for PC. (Default)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "BUILD_NOT_SET",
 *             description: Some(
 *                 "Build type not set yet.",
 *             ),
 *         },
 *     ],
 * }
 */
/// hkpMoppCode::BuildType
/// A byte describing if MOPP Data is organized into chunks (PS3) or not (PC)
#[binrw::binrw]
pub enum hkMoppCodeBuildType {
    #[brw(magic = 0_u8)]
    BuiltWithChunkSubdivision,
    #[brw(magic = 1_u8)]
    BuiltWithoutChunkSubdivision,
    #[brw(magic = 2_u8)]
    BuildNotSet,
}
/*
 * Enum {
 *     name: "PlatformID",
 *     storage: "uint",
 *     description: Some(
 *         "Target platform for NiPersistentSrcTextureRendererData (later than 30.1).",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "ANY",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "XENON",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "PS3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "DX9",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "WII",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "D3D10",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "UNKNOWN_6",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "UNKNOWN_7",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "UNKNOWN_8",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Target platform for NiPersistentSrcTextureRendererData (later than 30.1).
#[binrw::binrw]
pub enum PlatformID {
    #[brw(magic = 0_u32)]
    Any,
    #[brw(magic = 1_u32)]
    Xenon,
    #[brw(magic = 2_u32)]
    Ps3,
    #[brw(magic = 3_u32)]
    Dx9,
    #[brw(magic = 4_u32)]
    Wii,
    #[brw(magic = 5_u32)]
    D3d10,
    #[brw(magic = 6_u32)]
    Unknown6,
    #[brw(magic = 7_u32)]
    Unknown7,
    #[brw(magic = 8_u32)]
    Unknown8,
}
/*
 * Enum {
 *     name: "RendererID",
 *     storage: "uint",
 *     description: Some(
 *         "Target renderer for NiPersistentSrcTextureRendererData (until 30.1).",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "XBOX360",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "PS3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "DX9",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "D3D10",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "WII",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "GENERIC",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "D3D11",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Target renderer for NiPersistentSrcTextureRendererData (until 30.1).
#[binrw::binrw]
pub enum RendererID {
    #[brw(magic = 0_u32)]
    Xbox360,
    #[brw(magic = 1_u32)]
    Ps3,
    #[brw(magic = 2_u32)]
    Dx9,
    #[brw(magic = 3_u32)]
    D3d10,
    #[brw(magic = 4_u32)]
    Wii,
    #[brw(magic = 5_u32)]
    Generic,
    #[brw(magic = 6_u32)]
    D3d11,
}
/*
 * Enum {
 *     name: "PixelFormat",
 *     storage: "uint",
 *     description: Some(
 *         "Describes the pixel format used by the NiPixelData object to store a texture.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "FMT_RGB",
 *             description: Some(
 *                 "24-bit RGB. 8 bits per red, blue, and green component.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "FMT_RGBA",
 *             description: Some(
 *                 "32-bit RGB with alpha. 8 bits per red, blue, green, and alpha component.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "FMT_PAL",
 *             description: Some(
 *                 "8-bit palette index.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "FMT_PALA",
 *             description: Some(
 *                 "8-bit palette index with alpha.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "FMT_DXT1",
 *             description: Some(
 *                 "DXT1 compressed texture.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "FMT_DXT3",
 *             description: Some(
 *                 "DXT3 compressed texture.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "FMT_DXT5",
 *             description: Some(
 *                 "DXT5 compressed texture.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "FMT_RGB24NONINT",
 *             description: Some(
 *                 "(Deprecated) 24-bit noninterleaved texture, an old PS2 format.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "FMT_BUMP",
 *             description: Some(
 *                 "Uncompressed dU/dV gradient bump map.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "FMT_BUMPLUMA",
 *             description: Some(
 *                 "Uncompressed dU/dV gradient bump map with luma channel representing shininess.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "FMT_RENDERSPEC",
 *             description: Some(
 *                 "Generic descriptor for any renderer-specific format not described by other formats.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "11",
 *             name: "FMT_1CH",
 *             description: Some(
 *                 "Generic descriptor for formats with 1 component.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "12",
 *             name: "FMT_2CH",
 *             description: Some(
 *                 "Generic descriptor for formats with 2 components.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "13",
 *             name: "FMT_3CH",
 *             description: Some(
 *                 "Generic descriptor for formats with 3 components.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "14",
 *             name: "FMT_4CH",
 *             description: Some(
 *                 "Generic descriptor for formats with 4 components.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "15",
 *             name: "FMT_DEPTH_STENCIL",
 *             description: Some(
 *                 "Indicates the NiPixelFormat is meant to be used on a depth/stencil surface.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "16",
 *             name: "FMT_UNKNOWN",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Describes the pixel format used by the NiPixelData object to store a texture.
#[binrw::binrw]
pub enum PixelFormat {
    #[brw(magic = 0_u32)]
    FmtRgb,
    #[brw(magic = 1_u32)]
    FmtRgba,
    #[brw(magic = 2_u32)]
    FmtPal,
    #[brw(magic = 3_u32)]
    FmtPala,
    #[brw(magic = 4_u32)]
    FmtDxt1,
    #[brw(magic = 5_u32)]
    FmtDxt3,
    #[brw(magic = 6_u32)]
    FmtDxt5,
    #[brw(magic = 7_u32)]
    FmtRgb24nonint,
    #[brw(magic = 8_u32)]
    FmtBump,
    #[brw(magic = 9_u32)]
    FmtBumpluma,
    #[brw(magic = 10_u32)]
    FmtRenderspec,
    #[brw(magic = 11_u32)]
    Fmt1ch,
    #[brw(magic = 12_u32)]
    Fmt2ch,
    #[brw(magic = 13_u32)]
    Fmt3ch,
    #[brw(magic = 14_u32)]
    Fmt4ch,
    #[brw(magic = 15_u32)]
    FmtDepthStencil,
    #[brw(magic = 16_u32)]
    FmtUnknown,
}
/*
 * Enum {
 *     name: "PixelTiling",
 *     storage: "uint",
 *     description: Some(
 *         "Describes whether pixels have been tiled from their standard row-major format to a format optimized for a particular platform.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "TILE_NONE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "TILE_XENON",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "TILE_WII",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "TILE_NV_SWIZZLED",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Describes whether pixels have been tiled from their standard row-major format to a format optimized for a particular platform.
#[binrw::binrw]
pub enum PixelTiling {
    #[brw(magic = 0_u32)]
    TileNone,
    #[brw(magic = 1_u32)]
    TileXenon,
    #[brw(magic = 2_u32)]
    TileWii,
    #[brw(magic = 3_u32)]
    TileNvSwizzled,
}
/*
 * Enum {
 *     name: "PixelComponent",
 *     storage: "uint",
 *     description: Some(
 *         "Describes the pixel format used by the NiPixelData object to store a texture.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "COMP_RED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "COMP_GREEN",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "COMP_BLUE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "COMP_ALPHA",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "COMP_COMPRESSED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "COMP_OFFSET_U",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "COMP_OFFSET_V",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "COMP_OFFSET_W",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "COMP_OFFSET_Q",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "COMP_LUMA",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "COMP_HEIGHT",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "11",
 *             name: "COMP_VECTOR_X",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "12",
 *             name: "COMP_VECTOR_Y",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "13",
 *             name: "COMP_VECTOR_Z",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "14",
 *             name: "COMP_PADDING",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "15",
 *             name: "COMP_INTENSITY",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "16",
 *             name: "COMP_INDEX",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "17",
 *             name: "COMP_DEPTH",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "18",
 *             name: "COMP_STENCIL",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "19",
 *             name: "COMP_EMPTY",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Describes the pixel format used by the NiPixelData object to store a texture.
#[binrw::binrw]
pub enum PixelComponent {
    #[brw(magic = 0_u32)]
    CompRed,
    #[brw(magic = 1_u32)]
    CompGreen,
    #[brw(magic = 2_u32)]
    CompBlue,
    #[brw(magic = 3_u32)]
    CompAlpha,
    #[brw(magic = 4_u32)]
    CompCompressed,
    #[brw(magic = 5_u32)]
    CompOffsetU,
    #[brw(magic = 6_u32)]
    CompOffsetV,
    #[brw(magic = 7_u32)]
    CompOffsetW,
    #[brw(magic = 8_u32)]
    CompOffsetQ,
    #[brw(magic = 9_u32)]
    CompLuma,
    #[brw(magic = 10_u32)]
    CompHeight,
    #[brw(magic = 11_u32)]
    CompVectorX,
    #[brw(magic = 12_u32)]
    CompVectorY,
    #[brw(magic = 13_u32)]
    CompVectorZ,
    #[brw(magic = 14_u32)]
    CompPadding,
    #[brw(magic = 15_u32)]
    CompIntensity,
    #[brw(magic = 16_u32)]
    CompIndex,
    #[brw(magic = 17_u32)]
    CompDepth,
    #[brw(magic = 18_u32)]
    CompStencil,
    #[brw(magic = 19_u32)]
    CompEmpty,
}
/*
 * Enum {
 *     name: "PixelRepresentation",
 *     storage: "uint",
 *     description: Some(
 *         "Describes how each pixel should be accessed on NiPixelFormat.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "REP_NORM_INT",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "REP_HALF",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "REP_FLOAT",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "REP_INDEX",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "REP_COMPRESSED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "REP_UNKNOWN",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "REP_INT",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Describes how each pixel should be accessed on NiPixelFormat.
#[binrw::binrw]
pub enum PixelRepresentation {
    #[brw(magic = 0_u32)]
    RepNormInt,
    #[brw(magic = 1_u32)]
    RepHalf,
    #[brw(magic = 2_u32)]
    RepFloat,
    #[brw(magic = 3_u32)]
    RepIndex,
    #[brw(magic = 4_u32)]
    RepCompressed,
    #[brw(magic = 5_u32)]
    RepUnknown,
    #[brw(magic = 6_u32)]
    RepInt,
}
/*
 * Enum {
 *     name: "PixelLayout",
 *     storage: "uint",
 *     description: Some(
 *         "Describes the color depth in an NiTexture.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "LAY_PALETTIZED_8",
 *             description: Some(
 *                 "Texture is in 8-bit palettized format.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "LAY_HIGH_COLOR_16",
 *             description: Some(
 *                 "Texture is in 16-bit high color format.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "LAY_TRUE_COLOR_32",
 *             description: Some(
 *                 "Texture is in 32-bit true color format.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "LAY_COMPRESSED",
 *             description: Some(
 *                 "Texture is compressed.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "LAY_BUMPMAP",
 *             description: Some(
 *                 "Texture is a grayscale bump map.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "LAY_PALETTIZED_4",
 *             description: Some(
 *                 "Texture is in 4-bit palettized format.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "LAY_DEFAULT",
 *             description: Some(
 *                 "Use default setting.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "LAY_SINGLE_COLOR_8",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "LAY_SINGLE_COLOR_16",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "LAY_SINGLE_COLOR_32",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "LAY_DOUBLE_COLOR_32",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "11",
 *             name: "LAY_DOUBLE_COLOR_64",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "12",
 *             name: "LAY_FLOAT_COLOR_32",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "13",
 *             name: "LAY_FLOAT_COLOR_64",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "14",
 *             name: "LAY_FLOAT_COLOR_128",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "15",
 *             name: "LAY_SINGLE_COLOR_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "16",
 *             name: "LAY_DEPTH_24_X8",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Describes the color depth in an NiTexture.
#[binrw::binrw]
pub enum PixelLayout {
    #[brw(magic = 0_u32)]
    LayPalettized8,
    #[brw(magic = 1_u32)]
    LayHighColor16,
    #[brw(magic = 2_u32)]
    LayTrueColor32,
    #[brw(magic = 3_u32)]
    LayCompressed,
    #[brw(magic = 4_u32)]
    LayBumpmap,
    #[brw(magic = 5_u32)]
    LayPalettized4,
    #[brw(magic = 6_u32)]
    LayDefault,
    #[brw(magic = 7_u32)]
    LaySingleColor8,
    #[brw(magic = 8_u32)]
    LaySingleColor16,
    #[brw(magic = 9_u32)]
    LaySingleColor32,
    #[brw(magic = 10_u32)]
    LayDoubleColor32,
    #[brw(magic = 11_u32)]
    LayDoubleColor64,
    #[brw(magic = 12_u32)]
    LayFloatColor32,
    #[brw(magic = 13_u32)]
    LayFloatColor64,
    #[brw(magic = 14_u32)]
    LayFloatColor128,
    #[brw(magic = 15_u32)]
    LaySingleColor4,
    #[brw(magic = 16_u32)]
    LayDepth24X8,
}
/*
 * Enum {
 *     name: "MipMapFormat",
 *     storage: "uint",
 *     description: Some(
 *         "Describes how mipmaps are handled in an NiTexture.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "MIP_FMT_NO",
 *             description: Some(
 *                 "Texture does not use mip maps.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "MIP_FMT_YES",
 *             description: Some(
 *                 "Texture uses mip maps.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "MIP_FMT_DEFAULT",
 *             description: Some(
 *                 "Use default setting.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes how mipmaps are handled in an NiTexture.
#[binrw::binrw]
pub enum MipMapFormat {
    #[brw(magic = 0_u32)]
    MipFmtNo,
    #[brw(magic = 1_u32)]
    MipFmtYes,
    #[brw(magic = 2_u32)]
    MipFmtDefault,
}
/*
 * Enum {
 *     name: "AlphaFormat",
 *     storage: "uint",
 *     description: Some(
 *         "Describes how transparency is handled in an NiTexture.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "ALPHA_NONE",
 *             description: Some(
 *                 "No alpha.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "ALPHA_BINARY",
 *             description: Some(
 *                 "1-bit alpha.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "ALPHA_SMOOTH",
 *             description: Some(
 *                 "Interpolated 4- or 8-bit alpha.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "ALPHA_DEFAULT",
 *             description: Some(
 *                 "Use default setting.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes how transparency is handled in an NiTexture.
#[binrw::binrw]
pub enum AlphaFormat {
    #[brw(magic = 0_u32)]
    AlphaNone,
    #[brw(magic = 1_u32)]
    AlphaBinary,
    #[brw(magic = 2_u32)]
    AlphaSmooth,
    #[brw(magic = 3_u32)]
    AlphaDefault,
}
/*
 * Enum {
 *     name: "TexClampMode",
 *     storage: "uint",
 *     description: Some(
 *         "Describes the availiable texture clamp modes, i.e. the behavior of UV mapping outside the [0,1] range.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "CLAMP_S_CLAMP_T",
 *             description: Some(
 *                 "Clamp in both directions.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "CLAMP_S_WRAP_T",
 *             description: Some(
 *                 "Clamp in the S(U) direction but wrap in the T(V) direction.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "WRAP_S_CLAMP_T",
 *             description: Some(
 *                 "Wrap in the S(U) direction but clamp in the T(V) direction.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "WRAP_S_WRAP_T",
 *             description: Some(
 *                 "Wrap in both directions.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes the availiable texture clamp modes, i.e. the behavior of UV mapping outside the [0,1] range.
#[binrw::binrw]
pub enum TexClampMode {
    #[brw(magic = 0_u32)]
    ClampSClampT,
    #[brw(magic = 1_u32)]
    ClampSWrapT,
    #[brw(magic = 2_u32)]
    WrapSClampT,
    #[brw(magic = 3_u32)]
    WrapSWrapT,
}
/*
 * Enum {
 *     name: "TexFilterMode",
 *     storage: "uint",
 *     description: Some(
 *         "Describes the availiable texture filter modes, i.e. the way the pixels in a texture are displayed on screen.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "FILTER_NEAREST",
 *             description: Some(
 *                 "Nearest neighbor. Uses nearest texel with no mipmapping.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "FILTER_BILERP",
 *             description: Some(
 *                 "Bilinear. Linear interpolation with no mipmapping.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "FILTER_TRILERP",
 *             description: Some(
 *                 "Trilinear. Linear intepolation between 8 texels (4 nearest texels between 2 nearest mip levels).",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "FILTER_NEAREST_MIPNEAREST",
 *             description: Some(
 *                 "Nearest texel on nearest mip level.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "FILTER_NEAREST_MIPLERP",
 *             description: Some(
 *                 "Linear interpolates nearest texel between two nearest mip levels.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "FILTER_BILERP_MIPNEAREST",
 *             description: Some(
 *                 "Linear interpolates on nearest mip level.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "FILTER_ANISOTROPIC",
 *             description: Some(
 *                 "Anisotropic filtering. One or many trilinear samples depending on anisotropy.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes the availiable texture filter modes, i.e. the way the pixels in a texture are displayed on screen.
#[binrw::binrw]
pub enum TexFilterMode {
    #[brw(magic = 0_u32)]
    FilterNearest,
    #[brw(magic = 1_u32)]
    FilterBilerp,
    #[brw(magic = 2_u32)]
    FilterTrilerp,
    #[brw(magic = 3_u32)]
    FilterNearestMipnearest,
    #[brw(magic = 4_u32)]
    FilterNearestMiplerp,
    #[brw(magic = 5_u32)]
    FilterBilerpMipnearest,
    #[brw(magic = 6_u32)]
    FilterAnisotropic,
}
/*
 * Enum {
 *     name: "SourceVertexMode",
 *     storage: "uint",
 *     description: Some(
 *         "Describes how to apply vertex colors for NiVertexColorProperty.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "VERT_MODE_SRC_IGNORE",
 *             description: Some(
 *                 "Emissive, ambient, and diffuse colors are all specified by the NiMaterialProperty.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "VERT_MODE_SRC_EMISSIVE",
 *             description: Some(
 *                 "Emissive colors are specified by the source vertex colors. Ambient+Diffuse are specified by the NiMaterialProperty.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "VERT_MODE_SRC_AMB_DIF",
 *             description: Some(
 *                 "Ambient+Diffuse colors are specified by the source vertex colors. Emissive is specified by the NiMaterialProperty. (Default)",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes how to apply vertex colors for NiVertexColorProperty.
#[binrw::binrw]
pub enum SourceVertexMode {
    #[brw(magic = 0_u32)]
    VertModeSrcIgnore,
    #[brw(magic = 1_u32)]
    VertModeSrcEmissive,
    #[brw(magic = 2_u32)]
    VertModeSrcAmbDif,
}
/*
 * Enum {
 *     name: "LightingMode",
 *     storage: "uint",
 *     description: Some(
 *         "Describes which lighting equation components influence the final vertex color for NiVertexColorProperty.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "LIGHT_MODE_EMISSIVE",
 *             description: Some(
 *                 "Emissive.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "LIGHT_MODE_EMI_AMB_DIF",
 *             description: Some(
 *                 "Emissive + Ambient + Diffuse. (Default)",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes which lighting equation components influence the final vertex color for NiVertexColorProperty.
#[binrw::binrw]
pub enum LightingMode {
    #[brw(magic = 0_u32)]
    LightModeEmissive,
    #[brw(magic = 1_u32)]
    LightModeEmiAmbDif,
}
/*
 * Enum {
 *     name: "CycleType",
 *     storage: "uint",
 *     description: Some(
 *         "The animation cyle behavior.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "CYCLE_LOOP",
 *             description: Some(
 *                 "Loop",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "CYCLE_REVERSE",
 *             description: Some(
 *                 "Reverse",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "CYCLE_CLAMP",
 *             description: Some(
 *                 "Clamp",
 *             ),
 *         },
 *     ],
 * }
 */
/// The animation cyle behavior.
#[binrw::binrw]
pub enum CycleType {
    #[brw(magic = 0_u32)]
    CycleLoop,
    #[brw(magic = 1_u32)]
    CycleReverse,
    #[brw(magic = 2_u32)]
    CycleClamp,
}
/*
 * Enum {
 *     name: "FieldType",
 *     storage: "uint",
 *     description: Some(
 *         "The force field type.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "FIELD_WIND",
 *             description: Some(
 *                 "Wind (fixed direction)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "FIELD_POINT",
 *             description: Some(
 *                 "Point (fixed origin)",
 *             ),
 *         },
 *     ],
 * }
 */
/// The force field type.
#[binrw::binrw]
pub enum FieldType {
    #[brw(magic = 0_u32)]
    FieldWind,
    #[brw(magic = 1_u32)]
    FieldPoint,
}
/*
 * Enum {
 *     name: "BillboardMode",
 *     storage: "ushort",
 *     description: Some(
 *         "Determines the way the billboard will react to the camera.\r\n        Billboard mode is stored in lowest 3 bits although Oblivion vanilla nifs uses values higher than 7.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "ALWAYS_FACE_CAMERA",
 *             description: Some(
 *                 "Align billboard and camera forward vector. Minimized rotation.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "ROTATE_ABOUT_UP",
 *             description: Some(
 *                 "Align billboard and camera forward vector while allowing rotation around the up axis.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "RIGID_FACE_CAMERA",
 *             description: Some(
 *                 "Align billboard and camera forward vector. Non-minimized rotation.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "ALWAYS_FACE_CENTER",
 *             description: Some(
 *                 "Billboard forward vector always faces camera ceneter. Minimized rotation.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "RIGID_FACE_CENTER",
 *             description: Some(
 *                 "Billboard forward vector always faces camera ceneter. Non-minimized rotation.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "BSROTATE_ABOUT_UP",
 *             description: Some(
 *                 "The billboard will only rotate around its local Z axis (it always stays in its local X-Y plane).",
 *             ),
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "ROTATE_ABOUT_UP2",
 *             description: Some(
 *                 "The billboard will only rotate around the up axis (same as ROTATE_ABOUT_UP?).",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "UNKNOWN_8",
 *             description: Some(
 *                 "Found in Civ IV Gravebringer and Gravebringer_FX",
 *             ),
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "UNKNOWN_10",
 *             description: Some(
 *                 "Found in FO3 dlc04lighthouselightmech01",
 *             ),
 *         },
 *         EnumOption {
 *             value: "11",
 *             name: "UNKNOWN_11",
 *             description: Some(
 *                 "Found in Civ IV Afterworld_Boss_FX",
 *             ),
 *         },
 *         EnumOption {
 *             value: "12",
 *             name: "UNKNOWN_12",
 *             description: Some(
 *                 "Found in IRIS Online etc.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Determines the way the billboard will react to the camera.
/// Billboard mode is stored in lowest 3 bits although Oblivion vanilla nifs uses values higher than 7.
#[binrw::binrw]
pub enum BillboardMode {
    #[brw(magic = 0_u16)]
    AlwaysFaceCamera,
    #[brw(magic = 1_u16)]
    RotateAboutUp,
    #[brw(magic = 2_u16)]
    RigidFaceCamera,
    #[brw(magic = 3_u16)]
    AlwaysFaceCenter,
    #[brw(magic = 4_u16)]
    RigidFaceCenter,
    #[brw(magic = 5_u16)]
    BsrotateAboutUp,
    #[brw(magic = 9_u16)]
    RotateAboutUp2,
    #[brw(magic = 8_u16)]
    Unknown8,
    #[brw(magic = 10_u16)]
    Unknown10,
    #[brw(magic = 11_u16)]
    Unknown11,
    #[brw(magic = 12_u16)]
    Unknown12,
}
/*
 * Enum {
 *     name: "StencilTestFunc",
 *     storage: "uint",
 *     description: Some(
 *         "Describes stencil buffer test modes for NiStencilProperty.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "TEST_NEVER",
 *             description: Some(
 *                 "Always false. Ref value is ignored.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "TEST_LESS",
 *             description: Some(
 *                 "VRef ‹ VBuf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "TEST_EQUAL",
 *             description: Some(
 *                 "VRef = VBuf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "TEST_LESS_EQUAL",
 *             description: Some(
 *                 "VRef ≤ VBuf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "TEST_GREATER",
 *             description: Some(
 *                 "VRef › VBuf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "TEST_NOT_EQUAL",
 *             description: Some(
 *                 "VRef ≠ VBuf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "TEST_GREATER_EQUAL",
 *             description: Some(
 *                 "VRef ≥ VBuf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "TEST_ALWAYS",
 *             description: Some(
 *                 "Always true. Buffer is ignored.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes stencil buffer test modes for NiStencilProperty.
#[binrw::binrw]
pub enum StencilTestFunc {
    #[brw(magic = 0_u32)]
    TestNever,
    #[brw(magic = 1_u32)]
    TestLess,
    #[brw(magic = 2_u32)]
    TestEqual,
    #[brw(magic = 3_u32)]
    TestLessEqual,
    #[brw(magic = 4_u32)]
    TestGreater,
    #[brw(magic = 5_u32)]
    TestNotEqual,
    #[brw(magic = 6_u32)]
    TestGreaterEqual,
    #[brw(magic = 7_u32)]
    TestAlways,
}
/*
 * Enum {
 *     name: "StencilAction",
 *     storage: "uint",
 *     description: Some(
 *         "Describes the actions which can occur as a result of tests for NiStencilProperty.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "ACTION_KEEP",
 *             description: Some(
 *                 "Keep the current value in the stencil buffer.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "ACTION_ZERO",
 *             description: Some(
 *                 "Write zero to the stencil buffer.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "ACTION_REPLACE",
 *             description: Some(
 *                 "Write the reference value to the stencil buffer.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "ACTION_INCREMENT",
 *             description: Some(
 *                 "Increment the value in the stencil buffer.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "ACTION_DECREMENT",
 *             description: Some(
 *                 "Decrement the value in the stencil buffer.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "ACTION_INVERT",
 *             description: Some(
 *                 "Bitwise invert the value in the stencil buffer.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes the actions which can occur as a result of tests for NiStencilProperty.
#[binrw::binrw]
pub enum StencilAction {
    #[brw(magic = 0_u32)]
    ActionKeep,
    #[brw(magic = 1_u32)]
    ActionZero,
    #[brw(magic = 2_u32)]
    ActionReplace,
    #[brw(magic = 3_u32)]
    ActionIncrement,
    #[brw(magic = 4_u32)]
    ActionDecrement,
    #[brw(magic = 5_u32)]
    ActionInvert,
}
/*
 * Enum {
 *     name: "StencilDrawMode",
 *     storage: "uint",
 *     description: Some(
 *         "Describes the face culling options for NiStencilProperty.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "DRAW_CCW_OR_BOTH",
 *             description: Some(
 *                 "Application default, chooses between DRAW_CCW or DRAW_BOTH.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "DRAW_CCW",
 *             description: Some(
 *                 "Draw only the triangles whose vertices are ordered CCW with respect to the viewer. (Standard behavior)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "DRAW_CW",
 *             description: Some(
 *                 "Draw only the triangles whose vertices are ordered CW with respect to the viewer. (Effectively flips faces)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "DRAW_BOTH",
 *             description: Some(
 *                 "Draw all triangles, regardless of orientation. (Effectively force double-sided)",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes the face culling options for NiStencilProperty.
#[binrw::binrw]
pub enum StencilDrawMode {
    #[brw(magic = 0_u32)]
    DrawCcwOrBoth,
    #[brw(magic = 1_u32)]
    DrawCcw,
    #[brw(magic = 2_u32)]
    DrawCw,
    #[brw(magic = 3_u32)]
    DrawBoth,
}
/*
 * Enum {
 *     name: "TestFunction",
 *     storage: "uint",
 *     description: Some(
 *         "Describes Z-buffer test modes for NiZBufferProperty.\r\n        \"Less than\" = closer to camera, \"Greater than\" = further from camera.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "TEST_ALWAYS",
 *             description: Some(
 *                 "Always true. Buffer is ignored.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "TEST_LESS",
 *             description: Some(
 *                 "VRef ‹ VBuf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "TEST_EQUAL",
 *             description: Some(
 *                 "VRef = VBuf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "TEST_LESS_EQUAL",
 *             description: Some(
 *                 "VRef ≤ VBuf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "TEST_GREATER",
 *             description: Some(
 *                 "VRef › VBuf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "TEST_NOT_EQUAL",
 *             description: Some(
 *                 "VRef ≠ VBuf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "TEST_GREATER_EQUAL",
 *             description: Some(
 *                 "VRef ≥ VBuf",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "TEST_NEVER",
 *             description: Some(
 *                 "Always false. Ref value is ignored.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes Z-buffer test modes for NiZBufferProperty.
/// "Less than" = closer to camera, "Greater than" = further from camera.
#[binrw::binrw]
pub enum TestFunction {
    #[brw(magic = 0_u32)]
    TestAlways,
    #[brw(magic = 1_u32)]
    TestLess,
    #[brw(magic = 2_u32)]
    TestEqual,
    #[brw(magic = 3_u32)]
    TestLessEqual,
    #[brw(magic = 4_u32)]
    TestGreater,
    #[brw(magic = 5_u32)]
    TestNotEqual,
    #[brw(magic = 6_u32)]
    TestGreaterEqual,
    #[brw(magic = 7_u32)]
    TestNever,
}
/*
 * Enum {
 *     name: "AlphaFunction",
 *     storage: "ushort",
 *     description: Some(
 *         "Describes alpha blend modes for NiAlphaProperty.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "ONE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "ZERO",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "SRC_COLOR",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "INV_SRC_COLOR",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "DEST_COLOR",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "INV_DEST_COLOR",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "SRC_ALPHA",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "INV_SRC_ALPHA",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "DEST_ALPHA",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "INV_DEST_ALPHA",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "SRC_ALPHA_SATURATE",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Describes alpha blend modes for NiAlphaProperty.
#[binrw::binrw]
pub enum AlphaFunction {
    #[brw(magic = 0_u16)]
    One,
    #[brw(magic = 1_u16)]
    Zero,
    #[brw(magic = 2_u16)]
    SrcColor,
    #[brw(magic = 3_u16)]
    InvSrcColor,
    #[brw(magic = 4_u16)]
    DestColor,
    #[brw(magic = 5_u16)]
    InvDestColor,
    #[brw(magic = 6_u16)]
    SrcAlpha,
    #[brw(magic = 7_u16)]
    InvSrcAlpha,
    #[brw(magic = 8_u16)]
    DestAlpha,
    #[brw(magic = 9_u16)]
    InvDestAlpha,
    #[brw(magic = 10_u16)]
    SrcAlphaSaturate,
}
/*
 * Enum {
 *     name: "hkMotionType",
 *     storage: "byte",
 *     description: Some(
 *         "hkpMotion::MotionType. Motion type of a rigid body determines what happens when it is simulated.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "MO_SYS_INVALID",
 *             description: Some(
 *                 "Invalid",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "MO_SYS_DYNAMIC",
 *             description: Some(
 *                 "A fully-simulated, movable rigid body. At construction time the engine checks the input inertia and selects MO_SYS_SPHERE_INERTIA or MO_SYS_BOX_INERTIA as appropriate.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "MO_SYS_SPHERE_INERTIA",
 *             description: Some(
 *                 "Simulation is performed using a sphere inertia tensor.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "MO_SYS_SPHERE_STABILIZED",
 *             description: Some(
 *                 "This is the same as MO_SYS_SPHERE_INERTIA, except that simulation of the rigid body is \"softened\".",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "MO_SYS_BOX_INERTIA",
 *             description: Some(
 *                 "Simulation is performed using a box inertia tensor.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "MO_SYS_BOX_STABILIZED",
 *             description: Some(
 *                 "This is the same as MO_SYS_BOX_INERTIA, except that simulation of the rigid body is \"softened\".",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "MO_SYS_KEYFRAMED",
 *             description: Some(
 *                 "Simulation is not performed as a normal rigid body. The keyframed rigid body has an infinite mass when viewed by the rest of the system. (used for creatures)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "MO_SYS_FIXED",
 *             description: Some(
 *                 "This motion type is used for the static elements of a game scene, e.g. the landscape. Faster than MO_SYS_KEYFRAMED at velocity 0. (used for weapons)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "MO_SYS_THIN_BOX",
 *             description: Some(
 *                 "A box inertia motion which is optimized for thin boxes and has less stability problems",
 *             ),
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "MO_SYS_CHARACTER",
 *             description: Some(
 *                 "A specialized motion used for character controllers",
 *             ),
 *         },
 *     ],
 * }
 */
/// hkpMotion::MotionType. Motion type of a rigid body determines what happens when it is simulated.
#[binrw::binrw]
pub enum hkMotionType {
    #[brw(magic = 0_u8)]
    MoSysInvalid,
    #[brw(magic = 1_u8)]
    MoSysDynamic,
    #[brw(magic = 2_u8)]
    MoSysSphereInertia,
    #[brw(magic = 3_u8)]
    MoSysSphereStabilized,
    #[brw(magic = 4_u8)]
    MoSysBoxInertia,
    #[brw(magic = 5_u8)]
    MoSysBoxStabilized,
    #[brw(magic = 6_u8)]
    MoSysKeyframed,
    #[brw(magic = 7_u8)]
    MoSysFixed,
    #[brw(magic = 8_u8)]
    MoSysThinBox,
    #[brw(magic = 9_u8)]
    MoSysCharacter,
}
/*
 * Enum {
 *     name: "hkDeactivatorType",
 *     storage: "byte",
 *     description: Some(
 *         "hkpRigidBodyDeactivator::DeactivatorType. Deactivator Type determines which mechanism Havok will use to classify the body as deactivated.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "DEACTIVATOR_INVALID",
 *             description: Some(
 *                 "Invalid",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "DEACTIVATOR_NEVER",
 *             description: Some(
 *                 "This will force the rigid body to never deactivate.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "DEACTIVATOR_SPATIAL",
 *             description: Some(
 *                 "Tells Havok to use a spatial deactivation scheme. This makes use of high and low frequencies of positional motion to determine when deactivation should occur.",
 *             ),
 *         },
 *     ],
 * }
 */
/// hkpRigidBodyDeactivator::DeactivatorType. Deactivator Type determines which mechanism Havok will use to classify the body as deactivated.
#[binrw::binrw]
pub enum hkDeactivatorType {
    #[brw(magic = 0_u8)]
    DeactivatorInvalid,
    #[brw(magic = 1_u8)]
    DeactivatorNever,
    #[brw(magic = 2_u8)]
    DeactivatorSpatial,
}
/*
 * Enum {
 *     name: "hkSolverDeactivation",
 *     storage: "byte",
 *     description: Some(
 *         "hkpRigidBodyCinfo::SolverDeactivation.\r\n        A list of possible solver deactivation settings. This value defines how aggressively the solver deactivates objects.\r\n        Note: Solver deactivation does not save CPU, but reduces creeping of movable objects in a pile quite dramatically.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "SOLVER_DEACTIVATION_INVALID",
 *             description: Some(
 *                 "Invalid",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "SOLVER_DEACTIVATION_OFF",
 *             description: Some(
 *                 "No solver deactivation.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "SOLVER_DEACTIVATION_LOW",
 *             description: Some(
 *                 "Very conservative deactivation, typically no visible artifacts.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "SOLVER_DEACTIVATION_MEDIUM",
 *             description: Some(
 *                 "Normal deactivation, no serious visible artifacts in most cases.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "SOLVER_DEACTIVATION_HIGH",
 *             description: Some(
 *                 "Fast deactivation, visible artifacts.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "SOLVER_DEACTIVATION_MAX",
 *             description: Some(
 *                 "Very fast deactivation, visible artifacts.",
 *             ),
 *         },
 *     ],
 * }
 */
/// hkpRigidBodyCinfo::SolverDeactivation.
/// A list of possible solver deactivation settings. This value defines how aggressively the solver deactivates objects.
/// Note: Solver deactivation does not save CPU, but reduces creeping of movable objects in a pile quite dramatically.
#[binrw::binrw]
pub enum hkSolverDeactivation {
    #[brw(magic = 0_u8)]
    SolverDeactivationInvalid,
    #[brw(magic = 1_u8)]
    SolverDeactivationOff,
    #[brw(magic = 2_u8)]
    SolverDeactivationLow,
    #[brw(magic = 3_u8)]
    SolverDeactivationMedium,
    #[brw(magic = 4_u8)]
    SolverDeactivationHigh,
    #[brw(magic = 5_u8)]
    SolverDeactivationMax,
}
/*
 * Enum {
 *     name: "hkQualityType",
 *     storage: "byte",
 *     description: Some(
 *         "hkpCollidableQualityType. Describes the priority and quality of collisions for a body,\r\n            e.g. you may expect critical game play objects to have solid high-priority collisions so that they never sink into ground,\r\n            or may allow penetrations for visual debris objects.\r\n        Notes:\r\n            - Fixed and keyframed objects cannot interact with each other.\r\n            - Debris can interpenetrate but still responds to Bullet hits.\r\n            - Critical objects are forced to not interpenetrate.\r\n            - Moving objects can interpenetrate slightly with other Moving or Debris objects but nothing else.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "MO_QUAL_INVALID",
 *             description: Some(
 *                 "Automatically assigned to MO_QUAL_FIXED, MO_QUAL_KEYFRAMED or MO_QUAL_DEBRIS",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "MO_QUAL_FIXED",
 *             description: Some(
 *                 "Static body.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "MO_QUAL_KEYFRAMED",
 *             description: Some(
 *                 "Animated body with infinite mass.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "MO_QUAL_DEBRIS",
 *             description: Some(
 *                 "Low importance bodies adding visual detail.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "MO_QUAL_MOVING",
 *             description: Some(
 *                 "Moving bodies which should not penetrate or leave the world, but can.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "MO_QUAL_CRITICAL",
 *             description: Some(
 *                 "Gameplay critical bodies which cannot penetrate or leave the world under any circumstance.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "MO_QUAL_BULLET",
 *             description: Some(
 *                 "Fast-moving bodies, such as projectiles.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "MO_QUAL_USER",
 *             description: Some(
 *                 "For user.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "MO_QUAL_CHARACTER",
 *             description: Some(
 *                 "For use with rigid body character controllers.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "MO_QUAL_KEYFRAMED_REPORT",
 *             description: Some(
 *                 "Moving bodies with infinite mass which should report contact points and TOI collisions against all other bodies.",
 *             ),
 *         },
 *     ],
 * }
 */
/// hkpCollidableQualityType. Describes the priority and quality of collisions for a body,
/// e.g. you may expect critical game play objects to have solid high-priority collisions so that they never sink into ground,
/// or may allow penetrations for visual debris objects.
/// Notes:
/// - Fixed and keyframed objects cannot interact with each other.
/// - Debris can interpenetrate but still responds to Bullet hits.
/// - Critical objects are forced to not interpenetrate.
/// - Moving objects can interpenetrate slightly with other Moving or Debris objects but nothing else.
#[binrw::binrw]
pub enum hkQualityType {
    #[brw(magic = 0_u8)]
    MoQualInvalid,
    #[brw(magic = 1_u8)]
    MoQualFixed,
    #[brw(magic = 2_u8)]
    MoQualKeyframed,
    #[brw(magic = 3_u8)]
    MoQualDebris,
    #[brw(magic = 4_u8)]
    MoQualMoving,
    #[brw(magic = 5_u8)]
    MoQualCritical,
    #[brw(magic = 6_u8)]
    MoQualBullet,
    #[brw(magic = 7_u8)]
    MoQualUser,
    #[brw(magic = 8_u8)]
    MoQualCharacter,
    #[brw(magic = 9_u8)]
    MoQualKeyframedReport,
}
/*
 * Enum {
 *     name: "ForceType",
 *     storage: "uint",
 *     description: Some(
 *         "Describes the type of gravitational force.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "FORCE_PLANAR",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "FORCE_SPHERICAL",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "FORCE_UNKNOWN",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Describes the type of gravitational force.
#[binrw::binrw]
pub enum ForceType {
    #[brw(magic = 0_u32)]
    ForcePlanar,
    #[brw(magic = 1_u32)]
    ForceSpherical,
    #[brw(magic = 2_u32)]
    ForceUnknown,
}
/*
 * Enum {
 *     name: "TransformMember",
 *     storage: "uint",
 *     description: Some(
 *         "Describes which aspect of the NiTextureTransform the NiTextureTransformController will modify.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "TT_TRANSLATE_U",
 *             description: Some(
 *                 "Control the translation of the U coordinates.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "TT_TRANSLATE_V",
 *             description: Some(
 *                 "Control the translation of the V coordinates.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "TT_ROTATE",
 *             description: Some(
 *                 "Control the rotation of the coordinates.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "TT_SCALE_U",
 *             description: Some(
 *                 "Control the scale of the U coordinates.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "TT_SCALE_V",
 *             description: Some(
 *                 "Control the scale of the V coordinates.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes which aspect of the NiTextureTransform the NiTextureTransformController will modify.
#[binrw::binrw]
pub enum TransformMember {
    #[brw(magic = 0_u32)]
    TtTranslateU,
    #[brw(magic = 1_u32)]
    TtTranslateV,
    #[brw(magic = 2_u32)]
    TtRotate,
    #[brw(magic = 3_u32)]
    TtScaleU,
    #[brw(magic = 4_u32)]
    TtScaleV,
}
/*
 * Enum {
 *     name: "DecayType",
 *     storage: "uint",
 *     description: Some(
 *         "Describes the decay function of bomb forces.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "DECAY_NONE",
 *             description: Some(
 *                 "No decay.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "DECAY_LINEAR",
 *             description: Some(
 *                 "Linear decay.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "DECAY_EXPONENTIAL",
 *             description: Some(
 *                 "Exponential decay.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes the decay function of bomb forces.
#[binrw::binrw]
pub enum DecayType {
    #[brw(magic = 0_u32)]
    DecayNone,
    #[brw(magic = 1_u32)]
    DecayLinear,
    #[brw(magic = 2_u32)]
    DecayExponential,
}
/*
 * Enum {
 *     name: "SymmetryType",
 *     storage: "uint",
 *     description: Some(
 *         "Describes the symmetry type of bomb forces.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "SPHERICAL_SYMMETRY",
 *             description: Some(
 *                 "Spherical Symmetry.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "CYLINDRICAL_SYMMETRY",
 *             description: Some(
 *                 "Cylindrical Symmetry.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "PLANAR_SYMMETRY",
 *             description: Some(
 *                 "Planar Symmetry.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes the symmetry type of bomb forces.
#[binrw::binrw]
pub enum SymmetryType {
    #[brw(magic = 0_u32)]
    SphericalSymmetry,
    #[brw(magic = 1_u32)]
    CylindricalSymmetry,
    #[brw(magic = 2_u32)]
    PlanarSymmetry,
}
/*
 * Enum {
 *     name: "VelocityType",
 *     storage: "uint",
 *     description: Some(
 *         "Controls the way the a particle mesh emitter determines the starting speed and direction of the particles that are emitted.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "VELOCITY_USE_NORMALS",
 *             description: Some(
 *                 "Uses the normals of the meshes to determine staring velocity.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "VELOCITY_USE_RANDOM",
 *             description: Some(
 *                 "Starts particles with a random velocity.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "VELOCITY_USE_DIRECTION",
 *             description: Some(
 *                 "Uses the emission axis to determine initial particle direction?",
 *             ),
 *         },
 *     ],
 * }
 */
/// Controls the way the a particle mesh emitter determines the starting speed and direction of the particles that are emitted.
#[binrw::binrw]
pub enum VelocityType {
    #[brw(magic = 0_u32)]
    VelocityUseNormals,
    #[brw(magic = 1_u32)]
    VelocityUseRandom,
    #[brw(magic = 2_u32)]
    VelocityUseDirection,
}
/*
 * Enum {
 *     name: "EmitFrom",
 *     storage: "uint",
 *     description: Some(
 *         "Controls which parts of the mesh that the particles are emitted from.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "EMIT_FROM_VERTICES",
 *             description: Some(
 *                 "Emit particles from the vertices of the mesh.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "EMIT_FROM_FACE_CENTER",
 *             description: Some(
 *                 "Emit particles from the center of the faces of the mesh.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "EMIT_FROM_EDGE_CENTER",
 *             description: Some(
 *                 "Emit particles from the center of the edges of the mesh.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "EMIT_FROM_FACE_SURFACE",
 *             description: Some(
 *                 "Perhaps randomly emit particles from anywhere on the faces of the mesh?",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "EMIT_FROM_EDGE_SURFACE",
 *             description: Some(
 *                 "Perhaps randomly emit particles from anywhere on the edges of the mesh?",
 *             ),
 *         },
 *     ],
 * }
 */
/// Controls which parts of the mesh that the particles are emitted from.
#[binrw::binrw]
pub enum EmitFrom {
    #[brw(magic = 0_u32)]
    EmitFromVertices,
    #[brw(magic = 1_u32)]
    EmitFromFaceCenter,
    #[brw(magic = 2_u32)]
    EmitFromEdgeCenter,
    #[brw(magic = 3_u32)]
    EmitFromFaceSurface,
    #[brw(magic = 4_u32)]
    EmitFromEdgeSurface,
}
/*
 * Enum {
 *     name: "TextureType",
 *     storage: "uint",
 *     description: Some(
 *         "The type of information that is stored in a texture used by an NiTextureEffect.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "TEX_PROJECTED_LIGHT",
 *             description: Some(
 *                 "Apply a projected light texture. Each light effect is summed before multiplying by the base texture.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "TEX_PROJECTED_SHADOW",
 *             description: Some(
 *                 "Apply a projected shadow texture. Each shadow effect is multiplied by the base texture.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "TEX_ENVIRONMENT_MAP",
 *             description: Some(
 *                 "Apply an environment map texture. Added to the base texture and light/shadow/decal maps.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "TEX_FOG_MAP",
 *             description: Some(
 *                 "Apply a fog map texture. Alpha channel is used to blend the color channel with the base texture.",
 *             ),
 *         },
 *     ],
 * }
 */
/// The type of information that is stored in a texture used by an NiTextureEffect.
#[binrw::binrw]
pub enum TextureType {
    #[brw(magic = 0_u32)]
    TexProjectedLight,
    #[brw(magic = 1_u32)]
    TexProjectedShadow,
    #[brw(magic = 2_u32)]
    TexEnvironmentMap,
    #[brw(magic = 3_u32)]
    TexFogMap,
}
/*
 * Enum {
 *     name: "CoordGenType",
 *     storage: "uint",
 *     description: Some(
 *         "Determines the way that UV texture coordinates are generated.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "CG_WORLD_PARALLEL",
 *             description: Some(
 *                 "Use planar mapping.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "CG_WORLD_PERSPECTIVE",
 *             description: Some(
 *                 "Use perspective mapping.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "CG_SPHERE_MAP",
 *             description: Some(
 *                 "Use spherical mapping.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "CG_SPECULAR_CUBE_MAP",
 *             description: Some(
 *                 "Use specular cube mapping. For NiSourceCubeMap only.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "CG_DIFFUSE_CUBE_MAP",
 *             description: Some(
 *                 "Use diffuse cube mapping. For NiSourceCubeMap only.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Determines the way that UV texture coordinates are generated.
#[binrw::binrw]
pub enum CoordGenType {
    #[brw(magic = 0_u32)]
    CgWorldParallel,
    #[brw(magic = 1_u32)]
    CgWorldPerspective,
    #[brw(magic = 2_u32)]
    CgSphereMap,
    #[brw(magic = 3_u32)]
    CgSpecularCubeMap,
    #[brw(magic = 4_u32)]
    CgDiffuseCubeMap,
}
/*
 * Enum {
 *     name: "EndianType",
 *     storage: "byte",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "ENDIAN_BIG",
 *             description: Some(
 *                 "The numbers are stored in big endian format, such as those used by PowerPC Mac processors.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "ENDIAN_LITTLE",
 *             description: Some(
 *                 "The numbers are stored in little endian format, such as those used by Intel and AMD x86 processors.",
 *             ),
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum EndianType {
    #[brw(magic = 0_u8)]
    EndianBig,
    #[brw(magic = 1_u8)]
    EndianLittle,
}
/*
 * Enum {
 *     name: "MaterialColor",
 *     storage: "ushort",
 *     description: Some(
 *         "Used by NiMaterialColorControllers to select which type of color in the controlled object that will be animated.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "TC_AMBIENT",
 *             description: Some(
 *                 "Control the ambient color.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "TC_DIFFUSE",
 *             description: Some(
 *                 "Control the diffuse color.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "TC_SPECULAR",
 *             description: Some(
 *                 "Control the specular color.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "TC_SELF_ILLUM",
 *             description: Some(
 *                 "Control the self illumination color.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Used by NiMaterialColorControllers to select which type of color in the controlled object that will be animated.
#[binrw::binrw]
pub enum MaterialColor {
    #[brw(magic = 0_u16)]
    TcAmbient,
    #[brw(magic = 1_u16)]
    TcDiffuse,
    #[brw(magic = 2_u16)]
    TcSpecular,
    #[brw(magic = 3_u16)]
    TcSelfIllum,
}
/*
 * Enum {
 *     name: "LightColor",
 *     storage: "ushort",
 *     description: Some(
 *         "Used by NiLightColorControllers to select which type of color in the controlled object that will be animated.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "LC_DIFFUSE",
 *             description: Some(
 *                 "Control the diffuse color.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "LC_AMBIENT",
 *             description: Some(
 *                 "Control the ambient color.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Used by NiLightColorControllers to select which type of color in the controlled object that will be animated.
#[binrw::binrw]
pub enum LightColor {
    #[brw(magic = 0_u16)]
    LcDiffuse,
    #[brw(magic = 1_u16)]
    LcAmbient,
}
/*
 * Enum {
 *     name: "ConsistencyType",
 *     storage: "ushort",
 *     description: Some(
 *         "Used by NiGeometryData to control the volatility of the mesh.\r\n        Consistency Type is masked to only the upper 4 bits (0xF000). Dirty mask is the lower 12 (0x0FFF) but only used at runtime.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0x0000",
 *             name: "CT_MUTABLE",
 *             description: Some(
 *                 "Mutable Mesh",
 *             ),
 *         },
 *         EnumOption {
 *             value: "0x4000",
 *             name: "CT_STATIC",
 *             description: Some(
 *                 "Static Mesh",
 *             ),
 *         },
 *         EnumOption {
 *             value: "0x8000",
 *             name: "CT_VOLATILE",
 *             description: Some(
 *                 "Volatile Mesh",
 *             ),
 *         },
 *     ],
 * }
 */
/// Used by NiGeometryData to control the volatility of the mesh.
/// Consistency Type is masked to only the upper 4 bits (0xF000). Dirty mask is the lower 12 (0x0FFF) but only used at runtime.
#[binrw::binrw]
pub enum ConsistencyType {
    #[brw(magic = 0x0000_u16)]
    CtMutable,
    #[brw(magic = 0x4000_u16)]
    CtStatic,
    #[brw(magic = 0x8000_u16)]
    CtVolatile,
}
/*
 * Enum {
 *     name: "SortingMode",
 *     storage: "uint",
 *     description: Some(
 *         "Describes the way that NiSortAdjustNode modifies the sorting behavior for the subtree below it.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "SORTING_INHERIT",
 *             description: Some(
 *                 "Inherit. Acts identical to NiNode.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "SORTING_OFF",
 *             description: Some(
 *                 "Disables sort on all geometry under this node.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes the way that NiSortAdjustNode modifies the sorting behavior for the subtree below it.
#[binrw::binrw]
pub enum SortingMode {
    #[brw(magic = 0_u32)]
    SortingInherit,
    #[brw(magic = 1_u32)]
    SortingOff,
}
/*
 * Enum {
 *     name: "PropagationMode",
 *     storage: "uint",
 *     description: Some(
 *         "The propagation mode controls scene graph traversal during collision detection operations for NiCollisionData.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "PROPAGATE_ON_SUCCESS",
 *             description: Some(
 *                 "Propagation only occurs as a result of a successful collision.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "PROPAGATE_ON_FAILURE",
 *             description: Some(
 *                 "(Deprecated) Propagation only occurs as a result of a failed collision.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "PROPAGATE_ALWAYS",
 *             description: Some(
 *                 "Propagation always occurs regardless of collision result.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "PROPAGATE_NEVER",
 *             description: Some(
 *                 "Propagation never occurs regardless of collision result.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "PROPAGATE_UNKNOWN_6",
 *             description: Some(
 *                 "Propagation mode found in Civ IV Chariot_Celtic.",
 *             ),
 *         },
 *     ],
 * }
 */
/// The propagation mode controls scene graph traversal during collision detection operations for NiCollisionData.
#[binrw::binrw]
pub enum PropagationMode {
    #[brw(magic = 0_u32)]
    PropagateOnSuccess,
    #[brw(magic = 1_u32)]
    PropagateOnFailure,
    #[brw(magic = 2_u32)]
    PropagateAlways,
    #[brw(magic = 3_u32)]
    PropagateNever,
    #[brw(magic = 6_u32)]
    PropagateUnknown6,
}
/*
 * Enum {
 *     name: "CollisionMode",
 *     storage: "uint",
 *     description: Some(
 *         "The collision mode controls the type of collision operation that is to take place for NiCollisionData.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "USE_OBB",
 *             description: Some(
 *                 "Use Bounding Box",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "USE_TRI",
 *             description: Some(
 *                 "Use Triangles",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "USE_ABV",
 *             description: Some(
 *                 "Use Alternate Bounding Volumes",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "NOTEST",
 *             description: Some(
 *                 "Indicates that no collision test should be made.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "USE_NIBOUND",
 *             description: Some(
 *                 "Use NiBound",
 *             ),
 *         },
 *     ],
 * }
 */
/// The collision mode controls the type of collision operation that is to take place for NiCollisionData.
#[binrw::binrw]
pub enum CollisionMode {
    #[brw(magic = 0_u32)]
    UseObb,
    #[brw(magic = 1_u32)]
    UseTri,
    #[brw(magic = 2_u32)]
    UseAbv,
    #[brw(magic = 3_u32)]
    Notest,
    #[brw(magic = 4_u32)]
    UseNibound,
}
/*
 * Enum {
 *     name: "BoundVolumeType",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0xffffffff",
 *             name: "BASE_BV",
 *             description: Some(
 *                 "Default",
 *             ),
 *         },
 *         EnumOption {
 *             value: "0",
 *             name: "SPHERE_BV",
 *             description: Some(
 *                 "Sphere",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "BOX_BV",
 *             description: Some(
 *                 "Box",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "CAPSULE_BV",
 *             description: Some(
 *                 "Capsule",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "UNION_BV",
 *             description: Some(
 *                 "Union",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "HALFSPACE_BV",
 *             description: Some(
 *                 "Half Space",
 *             ),
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum BoundVolumeType {
    #[brw(magic = 0xffffffff_u32)]
    BaseBv,
    #[brw(magic = 0_u32)]
    SphereBv,
    #[brw(magic = 1_u32)]
    BoxBv,
    #[brw(magic = 2_u32)]
    CapsuleBv,
    #[brw(magic = 4_u32)]
    UnionBv,
    #[brw(magic = 5_u32)]
    HalfspaceBv,
}
/*
 * Enum {
 *     name: "hkResponseType",
 *     storage: "byte",
 *     description: Some(
 *         "hkpMaterial::ResponseType",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "RESPONSE_INVALID",
 *             description: Some(
 *                 "Invalid Response",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "RESPONSE_SIMPLE_CONTACT",
 *             description: Some(
 *                 "Do normal collision resolution",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "RESPONSE_REPORTING",
 *             description: Some(
 *                 "No collision resolution is performed but listeners are called",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "RESPONSE_NONE",
 *             description: Some(
 *                 "Do nothing, ignore all the results.",
 *             ),
 *         },
 *     ],
 * }
 */
/// hkpMaterial::ResponseType
#[binrw::binrw]
pub enum hkResponseType {
    #[brw(magic = 0_u8)]
    ResponseInvalid,
    #[brw(magic = 1_u8)]
    ResponseSimpleContact,
    #[brw(magic = 2_u8)]
    ResponseReporting,
    #[brw(magic = 3_u8)]
    ResponseNone,
}
/*
 * Enum {
 *     name: "BSDismemberBodyPartType",
 *     storage: "ushort",
 *     description: Some(
 *         "Biped bodypart data used for visibility control of triangles.  Options are Fallout 3, except where marked for Skyrim (uses SBP prefix)\r\n        Skyrim BP names are listed only for vanilla names, different creatures have different defnitions for naming.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "BP_TORSO",
 *             description: Some(
 *                 "Torso",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "BP_HEAD",
 *             description: Some(
 *                 "Head",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "BP_HEAD2",
 *             description: Some(
 *                 "Head 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "BP_LEFTARM",
 *             description: Some(
 *                 "Left Arm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "BP_LEFTARM2",
 *             description: Some(
 *                 "Left Arm 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "BP_RIGHTARM",
 *             description: Some(
 *                 "Right Arm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "BP_RIGHTARM2",
 *             description: Some(
 *                 "Right Arm 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "BP_LEFTLEG",
 *             description: Some(
 *                 "Left Leg",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "BP_LEFTLEG2",
 *             description: Some(
 *                 "Left Leg 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "BP_LEFTLEG3",
 *             description: Some(
 *                 "Left Leg 3",
 *             ),
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "BP_RIGHTLEG",
 *             description: Some(
 *                 "Right Leg",
 *             ),
 *         },
 *         EnumOption {
 *             value: "11",
 *             name: "BP_RIGHTLEG2",
 *             description: Some(
 *                 "Right Leg 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "12",
 *             name: "BP_RIGHTLEG3",
 *             description: Some(
 *                 "Right Leg 3",
 *             ),
 *         },
 *         EnumOption {
 *             value: "13",
 *             name: "BP_BRAIN",
 *             description: Some(
 *                 "Brain",
 *             ),
 *         },
 *         EnumOption {
 *             value: "30",
 *             name: "SBP_30_HEAD",
 *             description: Some(
 *                 "Skyrim, Head(Human), Body(Atronachs,Beasts), Mask(Dragonpriest)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "31",
 *             name: "SBP_31_HAIR",
 *             description: Some(
 *                 "Skyrim, Hair(human), Far(Dragon), Mask2(Dragonpriest),SkinnedFX(Spriggan)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "32",
 *             name: "SBP_32_BODY",
 *             description: Some(
 *                 "Skyrim, Main body, extras(Spriggan)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "33",
 *             name: "SBP_33_HANDS",
 *             description: Some(
 *                 "Skyrim, Hands L/R, BodyToo(Dragonpriest), Legs(Draugr), Arms(Giant)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "34",
 *             name: "SBP_34_FOREARMS",
 *             description: Some(
 *                 "Skyrim, Forearms L/R, Beard(Draugr)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "35",
 *             name: "SBP_35_AMULET",
 *             description: Some(
 *                 "Skyrim, Amulet",
 *             ),
 *         },
 *         EnumOption {
 *             value: "36",
 *             name: "SBP_36_RING",
 *             description: Some(
 *                 "Skyrim, Ring",
 *             ),
 *         },
 *         EnumOption {
 *             value: "37",
 *             name: "SBP_37_FEET",
 *             description: Some(
 *                 "Skyrim, Feet L/R",
 *             ),
 *         },
 *         EnumOption {
 *             value: "38",
 *             name: "SBP_38_CALVES",
 *             description: Some(
 *                 "Skyrim, Calves L/R",
 *             ),
 *         },
 *         EnumOption {
 *             value: "39",
 *             name: "SBP_39_SHIELD",
 *             description: Some(
 *                 "Skyrim, Shield",
 *             ),
 *         },
 *         EnumOption {
 *             value: "40",
 *             name: "SBP_40_TAIL",
 *             description: Some(
 *                 "Skyrim, Tail(Argonian/Khajiit), Skeleton01(Dragon), FX01(AtronachStorm),FXMist (Dragonpriest), Spit(Chaurus,Spider),SmokeFins(IceWraith)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "41",
 *             name: "SBP_41_LONGHAIR",
 *             description: Some(
 *                 "Skyrim, Long Hair(Human), Skeleton02(Dragon),FXParticles(Dragonpriest)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "42",
 *             name: "SBP_42_CIRCLET",
 *             description: Some(
 *                 "Skyrim, Circlet(Human, MouthFireEffect(Dragon)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "43",
 *             name: "SBP_43_EARS",
 *             description: Some(
 *                 "Skyrim, Ears",
 *             ),
 *         },
 *         EnumOption {
 *             value: "44",
 *             name: "SBP_44_DRAGON_BLOODHEAD_OR_MOD_MOUTH",
 *             description: Some(
 *                 "Skyrim, Bloodied dragon head, or NPC face/mouth",
 *             ),
 *         },
 *         EnumOption {
 *             value: "45",
 *             name: "SBP_45_DRAGON_BLOODWINGL_OR_MOD_NECK",
 *             description: Some(
 *                 "Skyrim, Left Bloodied dragon wing, Saddle(Horse), or NPC cape, scarf, shawl, neck-tie, etc.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "46",
 *             name: "SBP_46_DRAGON_BLOODWINGR_OR_MOD_CHEST_PRIMARY",
 *             description: Some(
 *                 "Skyrim, Right Bloodied dragon wing, or NPC chest primary or outergarment",
 *             ),
 *         },
 *         EnumOption {
 *             value: "47",
 *             name: "SBP_47_DRAGON_BLOODTAIL_OR_MOD_BACK",
 *             description: Some(
 *                 "Skyrim, Bloodied dragon tail, or NPC backpack/wings/...",
 *             ),
 *         },
 *         EnumOption {
 *             value: "48",
 *             name: "SBP_48_MOD_MISC1",
 *             description: Some(
 *                 "Anything that does not fit in the list",
 *             ),
 *         },
 *         EnumOption {
 *             value: "49",
 *             name: "SBP_49_MOD_PELVIS_PRIMARY",
 *             description: Some(
 *                 "Pelvis primary or outergarment",
 *             ),
 *         },
 *         EnumOption {
 *             value: "50",
 *             name: "SBP_50_DECAPITATEDHEAD",
 *             description: Some(
 *                 "Skyrim, Decapitated Head",
 *             ),
 *         },
 *         EnumOption {
 *             value: "51",
 *             name: "SBP_51_DECAPITATE",
 *             description: Some(
 *                 "Skyrim, Decapitate, neck gore",
 *             ),
 *         },
 *         EnumOption {
 *             value: "52",
 *             name: "SBP_52_MOD_PELVIS_SECONDARY",
 *             description: Some(
 *                 "Pelvis secondary or undergarment",
 *             ),
 *         },
 *         EnumOption {
 *             value: "53",
 *             name: "SBP_53_MOD_LEG_RIGHT",
 *             description: Some(
 *                 "Leg primary or outergarment or right leg",
 *             ),
 *         },
 *         EnumOption {
 *             value: "54",
 *             name: "SBP_54_MOD_LEG_LEFT",
 *             description: Some(
 *                 "Leg secondary or undergarment or left leg",
 *             ),
 *         },
 *         EnumOption {
 *             value: "55",
 *             name: "SBP_55_MOD_FACE_JEWELRY",
 *             description: Some(
 *                 "Face alternate or jewelry",
 *             ),
 *         },
 *         EnumOption {
 *             value: "56",
 *             name: "SBP_56_MOD_CHEST_SECONDARY",
 *             description: Some(
 *                 "Chest secondary or undergarment",
 *             ),
 *         },
 *         EnumOption {
 *             value: "57",
 *             name: "SBP_57_MOD_SHOULDER",
 *             description: Some(
 *                 "Shoulder",
 *             ),
 *         },
 *         EnumOption {
 *             value: "58",
 *             name: "SBP_58_MOD_ARM_LEFT",
 *             description: Some(
 *                 "Arm secondary or undergarment or left arm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "59",
 *             name: "SBP_59_MOD_ARM_RIGHT",
 *             description: Some(
 *                 "Arm primary or outergarment or right arm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "60",
 *             name: "SBP_60_MOD_MISC2",
 *             description: Some(
 *                 "Anything that does not fit in the list",
 *             ),
 *         },
 *         EnumOption {
 *             value: "61",
 *             name: "SBP_61_FX01",
 *             description: Some(
 *                 "Skyrim, FX01(Humanoid)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "101",
 *             name: "BP_SECTIONCAP_HEAD",
 *             description: Some(
 *                 "Section Cap | Head",
 *             ),
 *         },
 *         EnumOption {
 *             value: "102",
 *             name: "BP_SECTIONCAP_HEAD2",
 *             description: Some(
 *                 "Section Cap | Head 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "103",
 *             name: "BP_SECTIONCAP_LEFTARM",
 *             description: Some(
 *                 "Section Cap | Left Arm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "104",
 *             name: "BP_SECTIONCAP_LEFTARM2",
 *             description: Some(
 *                 "Section Cap | Left Arm 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "105",
 *             name: "BP_SECTIONCAP_RIGHTARM",
 *             description: Some(
 *                 "Section Cap | Right Arm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "106",
 *             name: "BP_SECTIONCAP_RIGHTARM2",
 *             description: Some(
 *                 "Section Cap | Right Arm 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "107",
 *             name: "BP_SECTIONCAP_LEFTLEG",
 *             description: Some(
 *                 "Section Cap | Left Leg",
 *             ),
 *         },
 *         EnumOption {
 *             value: "108",
 *             name: "BP_SECTIONCAP_LEFTLEG2",
 *             description: Some(
 *                 "Section Cap | Left Leg 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "109",
 *             name: "BP_SECTIONCAP_LEFTLEG3",
 *             description: Some(
 *                 "Section Cap | Left Leg 3",
 *             ),
 *         },
 *         EnumOption {
 *             value: "110",
 *             name: "BP_SECTIONCAP_RIGHTLEG",
 *             description: Some(
 *                 "Section Cap | Right Leg",
 *             ),
 *         },
 *         EnumOption {
 *             value: "111",
 *             name: "BP_SECTIONCAP_RIGHTLEG2",
 *             description: Some(
 *                 "Section Cap | Right Leg 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "112",
 *             name: "BP_SECTIONCAP_RIGHTLEG3",
 *             description: Some(
 *                 "Section Cap | Right Leg 3",
 *             ),
 *         },
 *         EnumOption {
 *             value: "113",
 *             name: "BP_SECTIONCAP_BRAIN",
 *             description: Some(
 *                 "Section Cap | Brain",
 *             ),
 *         },
 *         EnumOption {
 *             value: "130",
 *             name: "SBP_130_HEAD",
 *             description: Some(
 *                 "Skyrim, Head slot, use on full-face helmets",
 *             ),
 *         },
 *         EnumOption {
 *             value: "131",
 *             name: "SBP_131_HAIR",
 *             description: Some(
 *                 "Skyrim, Hair slot 1, use on hoods",
 *             ),
 *         },
 *         EnumOption {
 *             value: "132",
 *             name: "SBP_132_HAIR",
 *             description: Some(
 *                 "Skyrim, Hair slot 2?, use on hoods",
 *             ),
 *         },
 *         EnumOption {
 *             value: "141",
 *             name: "SBP_141_LONGHAIR",
 *             description: Some(
 *                 "Skyrim, Hair slot 2, use for longer hair",
 *             ),
 *         },
 *         EnumOption {
 *             value: "142",
 *             name: "SBP_142_CIRCLET",
 *             description: Some(
 *                 "Skyrim, Circlet slot 1, use for circlets",
 *             ),
 *         },
 *         EnumOption {
 *             value: "143",
 *             name: "SBP_143_EARS",
 *             description: Some(
 *                 "Skyrim, Ear slot",
 *             ),
 *         },
 *         EnumOption {
 *             value: "150",
 *             name: "SBP_150_DECAPITATEDHEAD",
 *             description: Some(
 *                 "Skyrim, neck gore on head side",
 *             ),
 *         },
 *         EnumOption {
 *             value: "201",
 *             name: "BP_TORSOCAP_HEAD",
 *             description: Some(
 *                 "Torso Cap | Head",
 *             ),
 *         },
 *         EnumOption {
 *             value: "202",
 *             name: "BP_TORSOCAP_HEAD2",
 *             description: Some(
 *                 "Torso Cap | Head 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "203",
 *             name: "BP_TORSOCAP_LEFTARM",
 *             description: Some(
 *                 "Torso Cap | Left Arm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "204",
 *             name: "BP_TORSOCAP_LEFTARM2",
 *             description: Some(
 *                 "Torso Cap | Left Arm 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "205",
 *             name: "BP_TORSOCAP_RIGHTARM",
 *             description: Some(
 *                 "Torso Cap | Right Arm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "206",
 *             name: "BP_TORSOCAP_RIGHTARM2",
 *             description: Some(
 *                 "Torso Cap | Right Arm 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "207",
 *             name: "BP_TORSOCAP_LEFTLEG",
 *             description: Some(
 *                 "Torso Cap | Left Leg",
 *             ),
 *         },
 *         EnumOption {
 *             value: "208",
 *             name: "BP_TORSOCAP_LEFTLEG2",
 *             description: Some(
 *                 "Torso Cap | Left Leg 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "209",
 *             name: "BP_TORSOCAP_LEFTLEG3",
 *             description: Some(
 *                 "Torso Cap | Left Leg 3",
 *             ),
 *         },
 *         EnumOption {
 *             value: "210",
 *             name: "BP_TORSOCAP_RIGHTLEG",
 *             description: Some(
 *                 "Torso Cap | Right Leg",
 *             ),
 *         },
 *         EnumOption {
 *             value: "211",
 *             name: "BP_TORSOCAP_RIGHTLEG2",
 *             description: Some(
 *                 "Torso Cap | Right Leg 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "212",
 *             name: "BP_TORSOCAP_RIGHTLEG3",
 *             description: Some(
 *                 "Torso Cap | Right Leg 3",
 *             ),
 *         },
 *         EnumOption {
 *             value: "213",
 *             name: "BP_TORSOCAP_BRAIN",
 *             description: Some(
 *                 "Torso Cap | Brain",
 *             ),
 *         },
 *         EnumOption {
 *             value: "230",
 *             name: "SBP_230_HEAD",
 *             description: Some(
 *                 "Skyrim, Head slot, use for neck on character head",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1000",
 *             name: "BP_TORSOSECTION_HEAD",
 *             description: Some(
 *                 "Torso Section | Head",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2000",
 *             name: "BP_TORSOSECTION_HEAD2",
 *             description: Some(
 *                 "Torso Section | Head 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3000",
 *             name: "BP_TORSOSECTION_LEFTARM",
 *             description: Some(
 *                 "Torso Section | Left Arm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4000",
 *             name: "BP_TORSOSECTION_LEFTARM2",
 *             description: Some(
 *                 "Torso Section | Left Arm 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5000",
 *             name: "BP_TORSOSECTION_RIGHTARM",
 *             description: Some(
 *                 "Torso Section | Right Arm",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6000",
 *             name: "BP_TORSOSECTION_RIGHTARM2",
 *             description: Some(
 *                 "Torso Section | Right Arm 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7000",
 *             name: "BP_TORSOSECTION_LEFTLEG",
 *             description: Some(
 *                 "Torso Section | Left Leg",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8000",
 *             name: "BP_TORSOSECTION_LEFTLEG2",
 *             description: Some(
 *                 "Torso Section | Left Leg 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "9000",
 *             name: "BP_TORSOSECTION_LEFTLEG3",
 *             description: Some(
 *                 "Torso Section | Left Leg 3",
 *             ),
 *         },
 *         EnumOption {
 *             value: "10000",
 *             name: "BP_TORSOSECTION_RIGHTLEG",
 *             description: Some(
 *                 "Torso Section | Right Leg",
 *             ),
 *         },
 *         EnumOption {
 *             value: "11000",
 *             name: "BP_TORSOSECTION_RIGHTLEG2",
 *             description: Some(
 *                 "Torso Section | Right Leg 2",
 *             ),
 *         },
 *         EnumOption {
 *             value: "12000",
 *             name: "BP_TORSOSECTION_RIGHTLEG3",
 *             description: Some(
 *                 "Torso Section | Right Leg 3",
 *             ),
 *         },
 *         EnumOption {
 *             value: "13000",
 *             name: "BP_TORSOSECTION_BRAIN",
 *             description: Some(
 *                 "Torso Section | Brain",
 *             ),
 *         },
 *     ],
 * }
 */
/// Biped bodypart data used for visibility control of triangles.  Options are Fallout 3, except where marked for Skyrim (uses SBP prefix)
/// Skyrim BP names are listed only for vanilla names, different creatures have different defnitions for naming.
#[binrw::binrw]
pub enum BSDismemberBodyPartType {
    #[brw(magic = 0_u16)]
    BpTorso,
    #[brw(magic = 1_u16)]
    BpHead,
    #[brw(magic = 2_u16)]
    BpHead2,
    #[brw(magic = 3_u16)]
    BpLeftarm,
    #[brw(magic = 4_u16)]
    BpLeftarm2,
    #[brw(magic = 5_u16)]
    BpRightarm,
    #[brw(magic = 6_u16)]
    BpRightarm2,
    #[brw(magic = 7_u16)]
    BpLeftleg,
    #[brw(magic = 8_u16)]
    BpLeftleg2,
    #[brw(magic = 9_u16)]
    BpLeftleg3,
    #[brw(magic = 10_u16)]
    BpRightleg,
    #[brw(magic = 11_u16)]
    BpRightleg2,
    #[brw(magic = 12_u16)]
    BpRightleg3,
    #[brw(magic = 13_u16)]
    BpBrain,
    #[brw(magic = 30_u16)]
    Sbp30Head,
    #[brw(magic = 31_u16)]
    Sbp31Hair,
    #[brw(magic = 32_u16)]
    Sbp32Body,
    #[brw(magic = 33_u16)]
    Sbp33Hands,
    #[brw(magic = 34_u16)]
    Sbp34Forearms,
    #[brw(magic = 35_u16)]
    Sbp35Amulet,
    #[brw(magic = 36_u16)]
    Sbp36Ring,
    #[brw(magic = 37_u16)]
    Sbp37Feet,
    #[brw(magic = 38_u16)]
    Sbp38Calves,
    #[brw(magic = 39_u16)]
    Sbp39Shield,
    #[brw(magic = 40_u16)]
    Sbp40Tail,
    #[brw(magic = 41_u16)]
    Sbp41Longhair,
    #[brw(magic = 42_u16)]
    Sbp42Circlet,
    #[brw(magic = 43_u16)]
    Sbp43Ears,
    #[brw(magic = 44_u16)]
    Sbp44DragonBloodheadOrModMouth,
    #[brw(magic = 45_u16)]
    Sbp45DragonBloodwinglOrModNeck,
    #[brw(magic = 46_u16)]
    Sbp46DragonBloodwingrOrModChestPrimary,
    #[brw(magic = 47_u16)]
    Sbp47DragonBloodtailOrModBack,
    #[brw(magic = 48_u16)]
    Sbp48ModMisc1,
    #[brw(magic = 49_u16)]
    Sbp49ModPelvisPrimary,
    #[brw(magic = 50_u16)]
    Sbp50Decapitatedhead,
    #[brw(magic = 51_u16)]
    Sbp51Decapitate,
    #[brw(magic = 52_u16)]
    Sbp52ModPelvisSecondary,
    #[brw(magic = 53_u16)]
    Sbp53ModLegRight,
    #[brw(magic = 54_u16)]
    Sbp54ModLegLeft,
    #[brw(magic = 55_u16)]
    Sbp55ModFaceJewelry,
    #[brw(magic = 56_u16)]
    Sbp56ModChestSecondary,
    #[brw(magic = 57_u16)]
    Sbp57ModShoulder,
    #[brw(magic = 58_u16)]
    Sbp58ModArmLeft,
    #[brw(magic = 59_u16)]
    Sbp59ModArmRight,
    #[brw(magic = 60_u16)]
    Sbp60ModMisc2,
    #[brw(magic = 61_u16)]
    Sbp61Fx01,
    #[brw(magic = 101_u16)]
    BpSectioncapHead,
    #[brw(magic = 102_u16)]
    BpSectioncapHead2,
    #[brw(magic = 103_u16)]
    BpSectioncapLeftarm,
    #[brw(magic = 104_u16)]
    BpSectioncapLeftarm2,
    #[brw(magic = 105_u16)]
    BpSectioncapRightarm,
    #[brw(magic = 106_u16)]
    BpSectioncapRightarm2,
    #[brw(magic = 107_u16)]
    BpSectioncapLeftleg,
    #[brw(magic = 108_u16)]
    BpSectioncapLeftleg2,
    #[brw(magic = 109_u16)]
    BpSectioncapLeftleg3,
    #[brw(magic = 110_u16)]
    BpSectioncapRightleg,
    #[brw(magic = 111_u16)]
    BpSectioncapRightleg2,
    #[brw(magic = 112_u16)]
    BpSectioncapRightleg3,
    #[brw(magic = 113_u16)]
    BpSectioncapBrain,
    #[brw(magic = 130_u16)]
    Sbp130Head,
    #[brw(magic = 131_u16)]
    Sbp131Hair,
    #[brw(magic = 132_u16)]
    Sbp132Hair,
    #[brw(magic = 141_u16)]
    Sbp141Longhair,
    #[brw(magic = 142_u16)]
    Sbp142Circlet,
    #[brw(magic = 143_u16)]
    Sbp143Ears,
    #[brw(magic = 150_u16)]
    Sbp150Decapitatedhead,
    #[brw(magic = 201_u16)]
    BpTorsocapHead,
    #[brw(magic = 202_u16)]
    BpTorsocapHead2,
    #[brw(magic = 203_u16)]
    BpTorsocapLeftarm,
    #[brw(magic = 204_u16)]
    BpTorsocapLeftarm2,
    #[brw(magic = 205_u16)]
    BpTorsocapRightarm,
    #[brw(magic = 206_u16)]
    BpTorsocapRightarm2,
    #[brw(magic = 207_u16)]
    BpTorsocapLeftleg,
    #[brw(magic = 208_u16)]
    BpTorsocapLeftleg2,
    #[brw(magic = 209_u16)]
    BpTorsocapLeftleg3,
    #[brw(magic = 210_u16)]
    BpTorsocapRightleg,
    #[brw(magic = 211_u16)]
    BpTorsocapRightleg2,
    #[brw(magic = 212_u16)]
    BpTorsocapRightleg3,
    #[brw(magic = 213_u16)]
    BpTorsocapBrain,
    #[brw(magic = 230_u16)]
    Sbp230Head,
    #[brw(magic = 1000_u16)]
    BpTorsosectionHead,
    #[brw(magic = 2000_u16)]
    BpTorsosectionHead2,
    #[brw(magic = 3000_u16)]
    BpTorsosectionLeftarm,
    #[brw(magic = 4000_u16)]
    BpTorsosectionLeftarm2,
    #[brw(magic = 5000_u16)]
    BpTorsosectionRightarm,
    #[brw(magic = 6000_u16)]
    BpTorsosectionRightarm2,
    #[brw(magic = 7000_u16)]
    BpTorsosectionLeftleg,
    #[brw(magic = 8000_u16)]
    BpTorsosectionLeftleg2,
    #[brw(magic = 9000_u16)]
    BpTorsosectionLeftleg3,
    #[brw(magic = 10000_u16)]
    BpTorsosectionRightleg,
    #[brw(magic = 11000_u16)]
    BpTorsosectionRightleg2,
    #[brw(magic = 12000_u16)]
    BpTorsosectionRightleg3,
    #[brw(magic = 13000_u16)]
    BpTorsosectionBrain,
}
/*
 * Enum {
 *     name: "BSLightingShaderType",
 *     storage: "uint",
 *     description: Some(
 *         "Values for configuring the shader type in a BSLightingShaderProperty",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "Default",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "Environment Map",
 *             description: Some(
 *                 "Enables EnvMap Mask(TS6), EnvMap Scale",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "Glow Shader",
 *             description: Some(
 *                 "Enables Glow(TS3)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "Parallax",
 *             description: Some(
 *                 "Enables Height(TS4)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "Face Tint",
 *             description: Some(
 *                 "Enables Detail(TS4), Tint(TS7)",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "Skin Tint",
 *             description: Some(
 *                 "Enables Skin Tint Color",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "Hair Tint",
 *             description: Some(
 *                 "Enables Hair Tint Color",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "Parallax Occ",
 *             description: Some(
 *                 "Enables Height(TS4), Max Passes, Scale. Unimplemented.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "Multitexture Landscape",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "LOD Landscape",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "Snow",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "11",
 *             name: "MultiLayer Parallax",
 *             description: Some(
 *                 "Enables EnvMap Mask(TS6), Layer(TS7), Parallax Layer Thickness, Parallax Refraction Scale, Parallax Inner Layer U Scale, Parallax Inner Layer V Scale, EnvMap Scale",
 *             ),
 *         },
 *         EnumOption {
 *             value: "12",
 *             name: "Tree Anim",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "13",
 *             name: "LOD Objects",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "14",
 *             name: "Sparkle Snow",
 *             description: Some(
 *                 "Enables SparkleParams",
 *             ),
 *         },
 *         EnumOption {
 *             value: "15",
 *             name: "LOD Objects HD",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "16",
 *             name: "Eye Envmap",
 *             description: Some(
 *                 "Enables EnvMap Mask(TS6), Eye EnvMap Scale",
 *             ),
 *         },
 *         EnumOption {
 *             value: "17",
 *             name: "Cloud",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "18",
 *             name: "LOD Landscape Noise",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "19",
 *             name: "Multitexture Landscape LOD Blend",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "20",
 *             name: "FO4 Dismemberment",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Values for configuring the shader type in a BSLightingShaderProperty
#[binrw::binrw]
pub enum BSLightingShaderType {
    #[brw(magic = 0_u32)]
    Default,
    #[brw(magic = 1_u32)]
    EnvironmentMap,
    #[brw(magic = 2_u32)]
    GlowShader,
    #[brw(magic = 3_u32)]
    Parallax,
    #[brw(magic = 4_u32)]
    FaceTint,
    #[brw(magic = 5_u32)]
    SkinTint,
    #[brw(magic = 6_u32)]
    HairTint,
    #[brw(magic = 7_u32)]
    ParallaxOcc,
    #[brw(magic = 8_u32)]
    MultitextureLandscape,
    #[brw(magic = 9_u32)]
    LodLandscape,
    #[brw(magic = 10_u32)]
    Snow,
    #[brw(magic = 11_u32)]
    MultilayerParallax,
    #[brw(magic = 12_u32)]
    TreeAnim,
    #[brw(magic = 13_u32)]
    LodObjects,
    #[brw(magic = 14_u32)]
    SparkleSnow,
    #[brw(magic = 15_u32)]
    LodObjectsHd,
    #[brw(magic = 16_u32)]
    EyeEnvmap,
    #[brw(magic = 17_u32)]
    Cloud,
    #[brw(magic = 18_u32)]
    LodLandscapeNoise,
    #[brw(magic = 19_u32)]
    MultitextureLandscapeLodBlend,
    #[brw(magic = 20_u32)]
    Fo4Dismemberment,
}
/*
 * Enum {
 *     name: "BSShaderType155",
 *     storage: "uint",
 *     description: Some(
 *         "Values for configuring the shader type in a BSLightingShaderProperty",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "Default",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "Glow",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "Face Tint",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "Skin Tint",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "Hair Tint",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "12",
 *             name: "Eye Envmap",
 *             description: Some(
 *                 "Enables EnvMap Mask, Eye EnvMap Scale",
 *             ),
 *         },
 *         EnumOption {
 *             value: "17",
 *             name: "Terrain",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Values for configuring the shader type in a BSLightingShaderProperty
#[binrw::binrw]
pub enum BSShaderType155 {
    #[brw(magic = 0_u32)]
    Default,
    #[brw(magic = 2_u32)]
    Glow,
    #[brw(magic = 3_u32)]
    FaceTint,
    #[brw(magic = 4_u32)]
    SkinTint,
    #[brw(magic = 5_u32)]
    HairTint,
    #[brw(magic = 12_u32)]
    EyeEnvmap,
    #[brw(magic = 17_u32)]
    Terrain,
}
/*
 * Enum {
 *     name: "EffectShaderControlledVariable",
 *     storage: "uint",
 *     description: Some(
 *         "An unsigned 32-bit integer, describing which float variable in BSEffectShaderProperty to animate.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "EmissiveMultiple",
 *             description: Some(
 *                 "EmissiveMultiple.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "Falloff Start Angle",
 *             description: Some(
 *                 "Falloff Start Angle (degrees).",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "Falloff Stop Angle",
 *             description: Some(
 *                 "Falloff Stop Angle (degrees).",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "Falloff Start Opacity",
 *             description: Some(
 *                 "Falloff Start Opacity.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "Falloff Stop Opacity",
 *             description: Some(
 *                 "Falloff Stop Opacity.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "Alpha Transparency",
 *             description: Some(
 *                 "Alpha Transparency (Emissive alpha?).",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "U Offset",
 *             description: Some(
 *                 "U Offset.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "U Scale",
 *             description: Some(
 *                 "U Scale.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "V Offset",
 *             description: Some(
 *                 "V Offset.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "V Scale",
 *             description: Some(
 *                 "V Scale.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "11",
 *             name: "Unknown 11",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "12",
 *             name: "Unknown 12",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "13",
 *             name: "Unknown 13",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "14",
 *             name: "Unknown 14",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// An unsigned 32-bit integer, describing which float variable in BSEffectShaderProperty to animate.
#[binrw::binrw]
pub enum EffectShaderControlledVariable {
    #[brw(magic = 0_u32)]
    Emissivemultiple,
    #[brw(magic = 1_u32)]
    FalloffStartAngle,
    #[brw(magic = 2_u32)]
    FalloffStopAngle,
    #[brw(magic = 3_u32)]
    FalloffStartOpacity,
    #[brw(magic = 4_u32)]
    FalloffStopOpacity,
    #[brw(magic = 5_u32)]
    AlphaTransparency,
    #[brw(magic = 6_u32)]
    UOffset,
    #[brw(magic = 7_u32)]
    UScale,
    #[brw(magic = 8_u32)]
    VOffset,
    #[brw(magic = 9_u32)]
    VScale,
    #[brw(magic = 11_u32)]
    Unknown11,
    #[brw(magic = 12_u32)]
    Unknown12,
    #[brw(magic = 13_u32)]
    Unknown13,
    #[brw(magic = 14_u32)]
    Unknown14,
}
/*
 * Enum {
 *     name: "EffectShaderControlledColor",
 *     storage: "uint",
 *     description: Some(
 *         "An unsigned 32-bit integer, describing which color in BSEffectShaderProperty to animate.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "Emissive Color",
 *             description: Some(
 *                 "Emissive Color.",
 *             ),
 *         },
 *     ],
 * }
 */
/// An unsigned 32-bit integer, describing which color in BSEffectShaderProperty to animate.
#[binrw::binrw]
pub enum EffectShaderControlledColor {
    #[brw(magic = 0_u32)]
    EmissiveColor,
}
/*
 * Enum {
 *     name: "LightingShaderControlledFloat",
 *     storage: "uint",
 *     description: Some(
 *         "An unsigned 32-bit integer, describing which float variable in BSLightingShaderProperty to animate.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "Refraction Strength",
 *             description: Some(
 *                 "The amount of distortion.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "Unknown 3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "Unknown 4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "Environment Map Scale",
 *             description: Some(
 *                 "Environment Map Scale.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "Glossiness",
 *             description: Some(
 *                 "Glossiness.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "Specular Strength",
 *             description: Some(
 *                 "Specular Strength.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "11",
 *             name: "Emissive Multiple",
 *             description: Some(
 *                 "Emissive Multiple.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "12",
 *             name: "Alpha",
 *             description: Some(
 *                 "Alpha.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "13",
 *             name: "Unknown 13",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "14",
 *             name: "Unknown 14",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "20",
 *             name: "U Offset",
 *             description: Some(
 *                 "U Offset.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "21",
 *             name: "U Scale",
 *             description: Some(
 *                 "U Scale.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "22",
 *             name: "V Offset",
 *             description: Some(
 *                 "V Offset.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "23",
 *             name: "V Scale",
 *             description: Some(
 *                 "V Scale.",
 *             ),
 *         },
 *     ],
 * }
 */
/// An unsigned 32-bit integer, describing which float variable in BSLightingShaderProperty to animate.
#[binrw::binrw]
pub enum LightingShaderControlledFloat {
    #[brw(magic = 0_u32)]
    RefractionStrength,
    #[brw(magic = 3_u32)]
    Unknown3,
    #[brw(magic = 4_u32)]
    Unknown4,
    #[brw(magic = 8_u32)]
    EnvironmentMapScale,
    #[brw(magic = 9_u32)]
    Glossiness,
    #[brw(magic = 10_u32)]
    SpecularStrength,
    #[brw(magic = 11_u32)]
    EmissiveMultiple,
    #[brw(magic = 12_u32)]
    Alpha,
    #[brw(magic = 13_u32)]
    Unknown13,
    #[brw(magic = 14_u32)]
    Unknown14,
    #[brw(magic = 20_u32)]
    UOffset,
    #[brw(magic = 21_u32)]
    UScale,
    #[brw(magic = 22_u32)]
    VOffset,
    #[brw(magic = 23_u32)]
    VScale,
}
/*
 * Enum {
 *     name: "LightingShaderControlledUShort",
 *     storage: "uint",
 *     description: Some(
 *         "An unsigned 32-bit integer, describing which integral value in BSLightingShaderProperty to animate.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "Unknown 1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "Unknown 2",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// An unsigned 32-bit integer, describing which integral value in BSLightingShaderProperty to animate.
#[binrw::binrw]
pub enum LightingShaderControlledUShort {
    #[brw(magic = 0_u32)]
    Unknown1,
    #[brw(magic = 1_u32)]
    Unknown2,
}
/*
 * Enum {
 *     name: "LightingShaderControlledColor",
 *     storage: "uint",
 *     description: Some(
 *         "An unsigned 32-bit integer, describing which color in BSLightingShaderProperty to animate.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "Specular Color",
 *             description: Some(
 *                 "Specular Color.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "Emissive Color",
 *             description: Some(
 *                 "Emissive Color.",
 *             ),
 *         },
 *     ],
 * }
 */
/// An unsigned 32-bit integer, describing which color in BSLightingShaderProperty to animate.
#[binrw::binrw]
pub enum LightingShaderControlledColor {
    #[brw(magic = 0_u32)]
    SpecularColor,
    #[brw(magic = 1_u32)]
    EmissiveColor,
}
/*
 * Enum {
 *     name: "hkConstraintType",
 *     storage: "uint",
 *     description: Some(
 *         "hkpConstraintData::ConstraintType. Describes the type of bhkConstraint.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "BallAndSocket",
 *             description: Some(
 *                 "A ball and socket constraint.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "Hinge",
 *             description: Some(
 *                 "A hinge constraint.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "Limited Hinge",
 *             description: Some(
 *                 "A limited hinge constraint.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "Prismatic",
 *             description: Some(
 *                 "A prismatic constraint.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "Ragdoll",
 *             description: Some(
 *                 "A ragdoll constraint.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "StiffSpring",
 *             description: Some(
 *                 "A stiff spring constraint.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "13",
 *             name: "Malleable",
 *             description: Some(
 *                 "A malleable constraint.",
 *             ),
 *         },
 *     ],
 * }
 */
/// hkpConstraintData::ConstraintType. Describes the type of bhkConstraint.
#[binrw::binrw]
pub enum hkConstraintType {
    #[brw(magic = 0_u32)]
    Ballandsocket,
    #[brw(magic = 1_u32)]
    Hinge,
    #[brw(magic = 2_u32)]
    LimitedHinge,
    #[brw(magic = 6_u32)]
    Prismatic,
    #[brw(magic = 7_u32)]
    Ragdoll,
    #[brw(magic = 8_u32)]
    Stiffspring,
    #[brw(magic = 13_u32)]
    Malleable,
}
/*
 * Enum {
 *     name: "FogFunction",
 *     storage: "ushort",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "FOG_Z_LINEAR",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "FOG_RANGE_SQ",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "FOG_VERTEX_ALPHA",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum FogFunction {
    #[brw(magic = 0_u16)]
    FogZLinear,
    #[brw(magic = 1_u16)]
    FogRangeSq,
    #[brw(magic = 2_u16)]
    FogVertexAlpha,
}
/*
 * Enum {
 *     name: "AnimType",
 *     storage: "ushort",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "APP_TIME",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "APP_INIT",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum AnimType {
    #[brw(magic = 0_u16)]
    AppTime,
    #[brw(magic = 1_u16)]
    AppInit,
}
/*
 * Enum {
 *     name: "DitherFlags",
 *     storage: "ushort",
 *     description: Some(
 *         "Flags for NiDitherProperty",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "DITHER_DISABLED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "DITHER_ENABLED",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Flags for NiDitherProperty
#[binrw::binrw]
pub enum DitherFlags {
    #[brw(magic = 0_u16)]
    DitherDisabled,
    #[brw(magic = 1_u16)]
    DitherEnabled,
}
/*
 * Enum {
 *     name: "ShadeFlags",
 *     storage: "ushort",
 *     description: Some(
 *         "Flags for NiShadeProperty",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "SHADING_HARD",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "SHADING_SMOOTH",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Flags for NiShadeProperty
#[binrw::binrw]
pub enum ShadeFlags {
    #[brw(magic = 0_u16)]
    ShadingHard,
    #[brw(magic = 1_u16)]
    ShadingSmooth,
}
/*
 * Enum {
 *     name: "SpecularFlags",
 *     storage: "ushort",
 *     description: Some(
 *         "Flags for NiSpecularProperty",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "SPECULAR_DISABLED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "SPECULAR_ENABLED",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Flags for NiSpecularProperty
#[binrw::binrw]
pub enum SpecularFlags {
    #[brw(magic = 0_u16)]
    SpecularDisabled,
    #[brw(magic = 1_u16)]
    SpecularEnabled,
}
/*
 * Enum {
 *     name: "WireframeFlags",
 *     storage: "ushort",
 *     description: Some(
 *         "Flags for NiWireframeProperty",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "WIREFRAME_DISABLED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "WIREFRAME_ENABLED",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Flags for NiWireframeProperty
#[binrw::binrw]
pub enum WireframeFlags {
    #[brw(magic = 0_u16)]
    WireframeDisabled,
    #[brw(magic = 1_u16)]
    WireframeEnabled,
}
/*
 * Enum {
 *     name: "GeomMorpherFlags",
 *     storage: "ushort",
 *     description: Some(
 *         "Flags for NiGeomMorpherController",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "UPDATE_NORMALS_DISABLED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "UPDATE_NORMALS_ENABLED",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Flags for NiGeomMorpherController
#[binrw::binrw]
pub enum GeomMorpherFlags {
    #[brw(magic = 0_u16)]
    UpdateNormalsDisabled,
    #[brw(magic = 1_u16)]
    UpdateNormalsEnabled,
}
/*
 * Enum {
 *     name: "AGDConsistencyType",
 *     storage: "byte",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "AGD_MUTABLE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "AGD_STATIC",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "AGD_VOLATILE",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum AGDConsistencyType {
    #[brw(magic = 0_u8)]
    AgdMutable,
    #[brw(magic = 1_u8)]
    AgdStatic,
    #[brw(magic = 2_u8)]
    AgdVolatile,
}
/*
 * Enum {
 *     name: "NiNBTMethod",
 *     storage: "ushort",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "NBT_METHOD_NONE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "NBT_METHOD_NDL",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "NBT_METHOD_MAX",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "NBT_METHOD_ATI",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NiNBTMethod {
    #[brw(magic = 0_u16)]
    NbtMethodNone,
    #[brw(magic = 1_u16)]
    NbtMethodNdl,
    #[brw(magic = 2_u16)]
    NbtMethodMax,
    #[brw(magic = 3_u16)]
    NbtMethodAti,
}
/*
 * Enum {
 *     name: "TransformMethod",
 *     storage: "uint",
 *     description: Some(
 *         "Describes the order of scaling and rotation matrices. Translate, Scale, Rotation, Center are from TexDesc.\r\n        Back = inverse of Center. FromMaya = inverse of the V axis with a positive translation along V of 1 unit.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "Maya Deprecated",
 *             description: Some(
 *                 "Center * Rotation * Back * Translate * Scale",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "Max",
 *             description: Some(
 *                 "Center * Scale * Rotation * Translate * Back",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "Maya",
 *             description: Some(
 *                 "Center * Rotation * Back * FromMaya * Translate * Scale",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes the order of scaling and rotation matrices. Translate, Scale, Rotation, Center are from TexDesc.
/// Back = inverse of Center. FromMaya = inverse of the V axis with a positive translation along V of 1 unit.
#[binrw::binrw]
pub enum TransformMethod {
    #[brw(magic = 0_u32)]
    MayaDeprecated,
    #[brw(magic = 1_u32)]
    Max,
    #[brw(magic = 2_u32)]
    Maya,
}
/*
 * Enum {
 *     name: "AnimationType",
 *     storage: "ushort",
 *     description: Some(
 *         "Bethesda Animation. Animation type used on this position. This specifies the function of this position.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "1",
 *             name: "Sit",
 *             description: Some(
 *                 "Actor use sit animation.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "Sleep",
 *             description: Some(
 *                 "Actor use sleep animation.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "Lean",
 *             description: Some(
 *                 "Used for lean animations?",
 *             ),
 *         },
 *     ],
 * }
 */
/// Bethesda Animation. Animation type used on this position. This specifies the function of this position.
#[binrw::binrw]
pub enum AnimationType {
    #[brw(magic = 1_u16)]
    Sit,
    #[brw(magic = 2_u16)]
    Sleep,
    #[brw(magic = 4_u16)]
    Lean,
}
/*
 * Enum {
 *     name: "ConstraintPriority",
 *     storage: "uint",
 *     description: Some(
 *         "hkpConstraintInstance::ConstraintPriority. Priority used for the constraint.\r\n        Values 2, 4, and 5 are unused or internal use only.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "PRIORITY_INVALID",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "PRIORITY_PSI",
 *             description: Some(
 *                 "Constraint is only solved at regular physics time steps.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "PRIORITY_TOI",
 *             description: Some(
 *                 "Constraint is also solved at time of impact events.",
 *             ),
 *         },
 *     ],
 * }
 */
/// hkpConstraintInstance::ConstraintPriority. Priority used for the constraint.
/// Values 2, 4, and 5 are unused or internal use only.
#[binrw::binrw]
pub enum ConstraintPriority {
    #[brw(magic = 0_u32)]
    PriorityInvalid,
    #[brw(magic = 1_u32)]
    PriorityPsi,
    #[brw(magic = 3_u32)]
    PriorityToi,
}
/*
 * Enum {
 *     name: "hkMotorType",
 *     storage: "byte",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "MOTOR_NONE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "MOTOR_POSITION",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "MOTOR_VELOCITY",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "MOTOR_SPRING",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum hkMotorType {
    #[brw(magic = 0_u8)]
    MotorNone,
    #[brw(magic = 1_u8)]
    MotorPosition,
    #[brw(magic = 2_u8)]
    MotorVelocity,
    #[brw(magic = 3_u8)]
    MotorSpring,
}
/*
 * Enum {
 *     name: "ImageType",
 *     storage: "uint",
 *     description: Some(
 *         "Determines how the raw image data is stored in NiRawImageData.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "1",
 *             name: "RGB",
 *             description: Some(
 *                 "Colors store red, blue, and green components.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "RGBA",
 *             description: Some(
 *                 "Colors store red, blue, green, and alpha components.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Determines how the raw image data is stored in NiRawImageData.
#[binrw::binrw]
pub enum ImageType {
    #[brw(magic = 1_u32)]
    Rgb,
    #[brw(magic = 2_u32)]
    Rgba,
}
/*
 * Enum {
 *     name: "BroadPhaseType",
 *     storage: "byte",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "BROAD_PHASE_INVALID",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "BROAD_PHASE_ENTITY",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "BROAD_PHASE_PHANTOM",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "BROAD_PHASE_BORDER",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum BroadPhaseType {
    #[brw(magic = 0_u8)]
    BroadPhaseInvalid,
    #[brw(magic = 1_u8)]
    BroadPhaseEntity,
    #[brw(magic = 2_u8)]
    BroadPhasePhantom,
    #[brw(magic = 3_u8)]
    BroadPhaseBorder,
}
/*
 * Enum {
 *     name: "NiPSysModifierOrder",
 *     storage: "uint",
 *     description: Some(
 *         "The set order for each derived class of NiPSysModifier.\r\n        Note: For Skyrim, BSPSysStripUpdateModifier is 8000 and for FO3 it is 2500.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "ORDER_KILLOLDPARTICLES",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "ORDER_BSLOD",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1000",
 *             name: "ORDER_EMITTER",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2000",
 *             name: "ORDER_SPAWN",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2500",
 *             name: "ORDER_FO3_BSSTRIPUPDATE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3000",
 *             name: "ORDER_GENERAL",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4000",
 *             name: "ORDER_FORCE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "5000",
 *             name: "ORDER_COLLIDER",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "6000",
 *             name: "ORDER_POS_UPDATE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "6500",
 *             name: "ORDER_POSTPOS_UPDATE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "6600",
 *             name: "ORDER_WORLDSHIFT_PARTSPAWN",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "7000",
 *             name: "ORDER_BOUND_UPDATE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "8000",
 *             name: "ORDER_SK_BSSTRIPUPDATE",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// The set order for each derived class of NiPSysModifier.
/// Note: For Skyrim, BSPSysStripUpdateModifier is 8000 and for FO3 it is 2500.
#[binrw::binrw]
pub enum NiPSysModifierOrder {
    #[brw(magic = 0_u32)]
    OrderKilloldparticles,
    #[brw(magic = 1_u32)]
    OrderBslod,
    #[brw(magic = 1000_u32)]
    OrderEmitter,
    #[brw(magic = 2000_u32)]
    OrderSpawn,
    #[brw(magic = 2500_u32)]
    OrderFo3Bsstripupdate,
    #[brw(magic = 3000_u32)]
    OrderGeneral,
    #[brw(magic = 4000_u32)]
    OrderForce,
    #[brw(magic = 5000_u32)]
    OrderCollider,
    #[brw(magic = 6000_u32)]
    OrderPosUpdate,
    #[brw(magic = 6500_u32)]
    OrderPostposUpdate,
    #[brw(magic = 6600_u32)]
    OrderWorldshiftPartspawn,
    #[brw(magic = 7000_u32)]
    OrderBoundUpdate,
    #[brw(magic = 8000_u32)]
    OrderSkBsstripupdate,
}
/*
 * Enum {
 *     name: "NiSceneDescNxBroadPhaseType",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "BROADPHASE_QUADRATIC",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "BROADPHASE_FULL",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "BROADPHASE_COHERENT",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NiSceneDescNxBroadPhaseType {
    #[brw(magic = 0_u32)]
    BroadphaseQuadratic,
    #[brw(magic = 1_u32)]
    BroadphaseFull,
    #[brw(magic = 2_u32)]
    BroadphaseCoherent,
}
/*
 * Enum {
 *     name: "NiSceneDescNxHwPipelineSpec",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "RB_PIPELINE_HLP_ONLY",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "PIPELINE_FULL",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "PIPELINE_DEBUG",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NiSceneDescNxHwPipelineSpec {
    #[brw(magic = 0_u32)]
    RbPipelineHlpOnly,
    #[brw(magic = 1_u32)]
    PipelineFull,
    #[brw(magic = 2_u32)]
    PipelineDebug,
}
/*
 * Enum {
 *     name: "NiSceneDescNxHwSceneType",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "SCENE_TYPE_RB",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "SCENE_TYPE_FLUID",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "SCENE_TYPE_FLUID_SOFTWARE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "SCENE_TYPE_CLOTH",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NiSceneDescNxHwSceneType {
    #[brw(magic = 0_u32)]
    SceneTypeRb,
    #[brw(magic = 1_u32)]
    SceneTypeFluid,
    #[brw(magic = 2_u32)]
    SceneTypeFluidSoftware,
    #[brw(magic = 3_u32)]
    SceneTypeCloth,
}
/*
 * Enum {
 *     name: "NxTimeStepMethod",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "TIMESTEP_FIXED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "TIMESTEP_VARIABLE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "TIMESTEP_INHERIT",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NxTimeStepMethod {
    #[brw(magic = 0_u32)]
    TimestepFixed,
    #[brw(magic = 1_u32)]
    TimestepVariable,
    #[brw(magic = 2_u32)]
    TimestepInherit,
}
/*
 * Enum {
 *     name: "NxSimulationType",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "SIMULATION_SW",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "SIMULATION_HW",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NxSimulationType {
    #[brw(magic = 0_u32)]
    SimulationSw,
    #[brw(magic = 1_u32)]
    SimulationHw,
}
/*
 * Enum {
 *     name: "NxBroadPhaseType",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "BP_TYPE_SAP_SINGLE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "BP_TYPE_SAP_MULTI",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NxBroadPhaseType {
    #[brw(magic = 0_u32)]
    BpTypeSapSingle,
    #[brw(magic = 1_u32)]
    BpTypeSapMulti,
}
/*
 * Enum {
 *     name: "NxFilterOp",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "FILTEROP_AND",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "FILTEROP_OR",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "FILTEROP_XOR",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "FILTEROP_NAND",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "FILTEROP_NOR",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "FILTEROP_NXOR",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "FILTEROP_SWAP_AND",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NxFilterOp {
    #[brw(magic = 0_u32)]
    FilteropAnd,
    #[brw(magic = 1_u32)]
    FilteropOr,
    #[brw(magic = 2_u32)]
    FilteropXor,
    #[brw(magic = 3_u32)]
    FilteropNand,
    #[brw(magic = 4_u32)]
    FilteropNor,
    #[brw(magic = 5_u32)]
    FilteropNxor,
    #[brw(magic = 6_u32)]
    FilteropSwapAnd,
}
/*
 * Enum {
 *     name: "NxThreadPriority",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "TP_HIGH",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "TP_ABOVE_NORMAL",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "TP_NORMAL",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "TP_BELOW_NORMAL",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "TP_LOW",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NxThreadPriority {
    #[brw(magic = 0_u32)]
    TpHigh,
    #[brw(magic = 1_u32)]
    TpAboveNormal,
    #[brw(magic = 2_u32)]
    TpNormal,
    #[brw(magic = 3_u32)]
    TpBelowNormal,
    #[brw(magic = 4_u32)]
    TpLow,
}
/*
 * Enum {
 *     name: "NxPruningStructure",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "PRUNING_NONE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "PRUNING_OCTREE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "PRUNING_QUADTREE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "PRUNING_DYNAMIC_AABB_TREE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "PRUNING_STATIC_AABB_TREE",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NxPruningStructure {
    #[brw(magic = 0_u32)]
    PruningNone,
    #[brw(magic = 1_u32)]
    PruningOctree,
    #[brw(magic = 2_u32)]
    PruningQuadtree,
    #[brw(magic = 3_u32)]
    PruningDynamicAabbTree,
    #[brw(magic = 4_u32)]
    PruningStaticAabbTree,
}
/*
 * Enum {
 *     name: "NxCompartmentType",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "SCT_RIGIDBODY",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "SCT_FLUID",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "SCT_CLOTH",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NxCompartmentType {
    #[brw(magic = 0_u32)]
    SctRigidbody,
    #[brw(magic = 1_u32)]
    SctFluid,
    #[brw(magic = 2_u32)]
    SctCloth,
}
/*
 * Enum {
 *     name: "NxDeviceCode",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "PPU_0",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "PPU_1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "PPU_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "PPU_3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "PPU_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "PPU_5",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "PPU_6",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "PPU_7",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "PPU_8",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4294901760",
 *             name: "CPU",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4294901761",
 *             name: "PPU_AUTO_ASSIGN",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NxDeviceCode {
    #[brw(magic = 0_u32)]
    Ppu0,
    #[brw(magic = 1_u32)]
    Ppu1,
    #[brw(magic = 2_u32)]
    Ppu2,
    #[brw(magic = 3_u32)]
    Ppu3,
    #[brw(magic = 4_u32)]
    Ppu4,
    #[brw(magic = 5_u32)]
    Ppu5,
    #[brw(magic = 6_u32)]
    Ppu6,
    #[brw(magic = 7_u32)]
    Ppu7,
    #[brw(magic = 8_u32)]
    Ppu8,
    #[brw(magic = 4294901760_u32)]
    Cpu,
    #[brw(magic = 4294901761_u32)]
    PpuAutoAssign,
}
/*
 * Enum {
 *     name: "NxJointType",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "PRISMATIC",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "REVOLUTE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "CYLINDRICAL",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "SPHERICAL",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "POINT_ON_LINE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "POINT_IN_PLANE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "DISTANCE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "PULLEY",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "FIXED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "D6",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NxJointType {
    #[brw(magic = 0_u32)]
    Prismatic,
    #[brw(magic = 1_u32)]
    Revolute,
    #[brw(magic = 2_u32)]
    Cylindrical,
    #[brw(magic = 3_u32)]
    Spherical,
    #[brw(magic = 4_u32)]
    PointOnLine,
    #[brw(magic = 5_u32)]
    PointInPlane,
    #[brw(magic = 6_u32)]
    Distance,
    #[brw(magic = 7_u32)]
    Pulley,
    #[brw(magic = 8_u32)]
    Fixed,
    #[brw(magic = 9_u32)]
    D6,
}
/*
 * Enum {
 *     name: "NxD6JointMotion",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "MOTION_LOCKED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "MOTION_LIMITED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "MOTION_FREE",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NxD6JointMotion {
    #[brw(magic = 0_u32)]
    MotionLocked,
    #[brw(magic = 1_u32)]
    MotionLimited,
    #[brw(magic = 2_u32)]
    MotionFree,
}
/*
 * Enum {
 *     name: "NxD6JointDriveType",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "1",
 *             name: "DRIVE_POSITION",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "DRIVE_VELOCITY",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NxD6JointDriveType {
    #[brw(magic = 1_u32)]
    DrivePosition,
    #[brw(magic = 2_u32)]
    DriveVelocity,
}
/*
 * Enum {
 *     name: "NxJointProjectionMode",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "JPM_NONE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "JPM_POINT_MINDIST",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "JPM_LINEAR_MINDIST",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NxJointProjectionMode {
    #[brw(magic = 0_u32)]
    JpmNone,
    #[brw(magic = 1_u32)]
    JpmPointMindist,
    #[brw(magic = 2_u32)]
    JpmLinearMindist,
}
/*
 * Enum {
 *     name: "NxShapeType",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "SHAPE_PLANE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "SHAPE_SPHERE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "SHAPE_BOX",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "SHAPE_CAPSULE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "SHAPE_WHEEL",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "SHAPE_CONVEX",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "SHAPE_MESH",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "SHAPE_HEIGHTFIELD",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "SHAPE_RAW_MESH",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "SHAPE_COMPOUND",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NxShapeType {
    #[brw(magic = 0_u32)]
    ShapePlane,
    #[brw(magic = 1_u32)]
    ShapeSphere,
    #[brw(magic = 2_u32)]
    ShapeBox,
    #[brw(magic = 3_u32)]
    ShapeCapsule,
    #[brw(magic = 4_u32)]
    ShapeWheel,
    #[brw(magic = 5_u32)]
    ShapeConvex,
    #[brw(magic = 6_u32)]
    ShapeMesh,
    #[brw(magic = 7_u32)]
    ShapeHeightfield,
    #[brw(magic = 8_u32)]
    ShapeRawMesh,
    #[brw(magic = 9_u32)]
    ShapeCompound,
}
/*
 * Enum {
 *     name: "NxCombineMode",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "AVERAGE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "MIN",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "MULTIPLY",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "MAX",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum NxCombineMode {
    #[brw(magic = 0_u32)]
    Average,
    #[brw(magic = 1_u32)]
    Min,
    #[brw(magic = 2_u32)]
    Multiply,
    #[brw(magic = 3_u32)]
    Max,
}
/*
 * Enum {
 *     name: "BSShaderType",
 *     storage: "uint",
 *     description: Some(
 *         "FO3 Shader Type",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "SHADER_TALL_GRASS",
 *             description: Some(
 *                 "Tall Grass Shader",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "SHADER_DEFAULT",
 *             description: Some(
 *                 "Standard Lighting Shader",
 *             ),
 *         },
 *         EnumOption {
 *             value: "10",
 *             name: "SHADER_SKY",
 *             description: Some(
 *                 "Sky Shader",
 *             ),
 *         },
 *         EnumOption {
 *             value: "14",
 *             name: "SHADER_SKIN",
 *             description: Some(
 *                 "Skin Shader",
 *             ),
 *         },
 *         EnumOption {
 *             value: "15",
 *             name: "SHADER_UNKNOWN",
 *             description: Some(
 *                 "scolbld06georgetown01",
 *             ),
 *         },
 *         EnumOption {
 *             value: "17",
 *             name: "SHADER_WATER",
 *             description: Some(
 *                 "Water Shader",
 *             ),
 *         },
 *         EnumOption {
 *             value: "29",
 *             name: "SHADER_LIGHTING30",
 *             description: Some(
 *                 "Lighting 3.0 Shader",
 *             ),
 *         },
 *         EnumOption {
 *             value: "32",
 *             name: "SHADER_TILE",
 *             description: Some(
 *                 "Tiled Shader",
 *             ),
 *         },
 *         EnumOption {
 *             value: "33",
 *             name: "SHADER_NOLIGHTING",
 *             description: Some(
 *                 "No Lighting Shader",
 *             ),
 *         },
 *     ],
 * }
 */
/// FO3 Shader Type
#[binrw::binrw]
pub enum BSShaderType {
    #[brw(magic = 0_u32)]
    ShaderTallGrass,
    #[brw(magic = 1_u32)]
    ShaderDefault,
    #[brw(magic = 10_u32)]
    ShaderSky,
    #[brw(magic = 14_u32)]
    ShaderSkin,
    #[brw(magic = 15_u32)]
    ShaderUnknown,
    #[brw(magic = 17_u32)]
    ShaderWater,
    #[brw(magic = 29_u32)]
    ShaderLighting30,
    #[brw(magic = 32_u32)]
    ShaderTile,
    #[brw(magic = 33_u32)]
    ShaderNolighting,
}
/*
 * Enum {
 *     name: "SkyObjectType",
 *     storage: "uint",
 *     description: Some(
 *         "Sets what sky function this object fulfills in BSSkyShaderProperty or SkyShaderProperty.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "BSSM_SKY_TEXTURE",
 *             description: Some(
 *                 "BSSM_Sky_Texture",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "BSSM_SKY_SUNGLARE",
 *             description: Some(
 *                 "BSSM_Sky_Sunglare",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "BSSM_SKY",
 *             description: Some(
 *                 "BSSM_Sky",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "BSSM_SKY_CLOUDS",
 *             description: Some(
 *                 "BSSM_Sky_Clouds",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "BSSM_SKY_STARS",
 *             description: Some(
 *                 "BSSM_Sky_Stars",
 *             ),
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "BSSM_SKY_MOON_STARS_MASK",
 *             description: Some(
 *                 "BSSM_Sky_Moon_Stars_Mask",
 *             ),
 *         },
 *     ],
 * }
 */
/// Sets what sky function this object fulfills in BSSkyShaderProperty or SkyShaderProperty.
#[binrw::binrw]
pub enum SkyObjectType {
    #[brw(magic = 0_u32)]
    BssmSkyTexture,
    #[brw(magic = 1_u32)]
    BssmSkySunglare,
    #[brw(magic = 2_u32)]
    BssmSky,
    #[brw(magic = 3_u32)]
    BssmSkyClouds,
    #[brw(magic = 5_u32)]
    BssmSkyStars,
    #[brw(magic = 7_u32)]
    BssmSkyMoonStarsMask,
}
/*
 * Enum {
 *     name: "BSShaderCRC32",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "1563274220",
 *             name: "CAST_SHADOWS",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1740048692",
 *             name: "ZBUFFER_TEST",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3166356979",
 *             name: "ZBUFFER_WRITE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "759557230",
 *             name: "TWO_SIDED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "348504749",
 *             name: "VERTEXCOLORS",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "731263983",
 *             name: "PBR",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3744563888",
 *             name: "SKINNED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2893749418",
 *             name: "ENVMAP",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2333069810",
 *             name: "VERTEX_ALPHA",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "314919375",
 *             name: "FACE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "442246519",
 *             name: "GRAYSCALE_TO_PALETTE_COLOR",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3849131744",
 *             name: "DECAL",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1576614759",
 *             name: "DYNAMIC_DECAL",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1264105798",
 *             name: "HAIRTINT",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1483897208",
 *             name: "SKIN_TINT",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2262553490",
 *             name: "EMIT_ENABLED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2399422528",
 *             name: "GLOWMAP",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1957349758",
 *             name: "REFRACTION",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "902349195",
 *             name: "REFRACTION_FALLOFF",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2994043788",
 *             name: "NOFADE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3030867718",
 *             name: "INVERTED_FADE_PATTERN",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3448946507",
 *             name: "RGB_FALLOFF",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2150459555",
 *             name: "EXTERNAL_EMITTANCE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2548465567",
 *             name: "MODELSPACENORMALS",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3196772338",
 *             name: "TRANSFORM_CHANGED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3473438218",
 *             name: "EFFECT_LIGHTING",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3980660124",
 *             name: "FALLOFF",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3503164976",
 *             name: "SOFT_EFFECT",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2901038324",
 *             name: "GRAYSCALE_TO_PALETTE_ALPHA",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2078326675",
 *             name: "WEAPON_BLOOD",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2896726515",
 *             name: "LOD_OBJECTS",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3707406987",
 *             name: "NO_EXPOSURE",
 *             description: None,
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum BSShaderCRC32 {
    #[brw(magic = 1563274220_u32)]
    CastShadows,
    #[brw(magic = 1740048692_u32)]
    ZbufferTest,
    #[brw(magic = 3166356979_u32)]
    ZbufferWrite,
    #[brw(magic = 759557230_u32)]
    TwoSided,
    #[brw(magic = 348504749_u32)]
    Vertexcolors,
    #[brw(magic = 731263983_u32)]
    Pbr,
    #[brw(magic = 3744563888_u32)]
    Skinned,
    #[brw(magic = 2893749418_u32)]
    Envmap,
    #[brw(magic = 2333069810_u32)]
    VertexAlpha,
    #[brw(magic = 314919375_u32)]
    Face,
    #[brw(magic = 442246519_u32)]
    GrayscaleToPaletteColor,
    #[brw(magic = 3849131744_u32)]
    Decal,
    #[brw(magic = 1576614759_u32)]
    DynamicDecal,
    #[brw(magic = 1264105798_u32)]
    Hairtint,
    #[brw(magic = 1483897208_u32)]
    SkinTint,
    #[brw(magic = 2262553490_u32)]
    EmitEnabled,
    #[brw(magic = 2399422528_u32)]
    Glowmap,
    #[brw(magic = 1957349758_u32)]
    Refraction,
    #[brw(magic = 902349195_u32)]
    RefractionFalloff,
    #[brw(magic = 2994043788_u32)]
    Nofade,
    #[brw(magic = 3030867718_u32)]
    InvertedFadePattern,
    #[brw(magic = 3448946507_u32)]
    RgbFalloff,
    #[brw(magic = 2150459555_u32)]
    ExternalEmittance,
    #[brw(magic = 2548465567_u32)]
    Modelspacenormals,
    #[brw(magic = 3196772338_u32)]
    TransformChanged,
    #[brw(magic = 3473438218_u32)]
    EffectLighting,
    #[brw(magic = 3980660124_u32)]
    Falloff,
    #[brw(magic = 3503164976_u32)]
    SoftEffect,
    #[brw(magic = 2901038324_u32)]
    GrayscaleToPaletteAlpha,
    #[brw(magic = 2078326675_u32)]
    WeaponBlood,
    #[brw(magic = 2896726515_u32)]
    LodObjects,
    #[brw(magic = 3707406987_u32)]
    NoExposure,
}
/*
 * Enum {
 *     name: "AnimNoteType",
 *     storage: "uint",
 *     description: Some(
 *         "Anim note types.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "ANT_INVALID",
 *             description: Some(
 *                 "ANT_INVALID",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "ANT_GRABIK",
 *             description: Some(
 *                 "ANT_GRABIK",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "ANT_LOOKIK",
 *             description: Some(
 *                 "ANT_LOOKIK",
 *             ),
 *         },
 *     ],
 * }
 */
/// Anim note types.
#[binrw::binrw]
pub enum AnimNoteType {
    #[brw(magic = 0_u32)]
    AntInvalid,
    #[brw(magic = 1_u32)]
    AntGrabik,
    #[brw(magic = 2_u32)]
    AntLookik,
}
/*
 * Enum {
 *     name: "BSCPCullingType",
 *     storage: "uint",
 *     description: Some(
 *         "Culling modes for multi bound nodes.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "CULL_NORMAL",
 *             description: Some(
 *                 "Normal",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "CULL_ALLPASS",
 *             description: Some(
 *                 "All Pass",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "CULL_ALLFAIL",
 *             description: Some(
 *                 "All Fail",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "CULL_IGNOREMULTIBOUNDS",
 *             description: Some(
 *                 "Ignore Multi Bounds",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "CULL_FORCEMULTIBOUNDSNOUPDATE",
 *             description: Some(
 *                 "Force Multi Bounds No Update",
 *             ),
 *         },
 *     ],
 * }
 */
/// Culling modes for multi bound nodes.
#[binrw::binrw]
pub enum BSCPCullingType {
    #[brw(magic = 0_u32)]
    CullNormal,
    #[brw(magic = 1_u32)]
    CullAllpass,
    #[brw(magic = 2_u32)]
    CullAllfail,
    #[brw(magic = 3_u32)]
    CullIgnoremultibounds,
    #[brw(magic = 4_u32)]
    CullForcemultiboundsnoupdate,
}
/*
 * Enum {
 *     name: "CloningBehavior",
 *     storage: "uint",
 *     description: Some(
 *         "Sets how objects are to be cloned.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "CLONING_SHARE",
 *             description: Some(
 *                 "Share this object pointer with the newly cloned scene.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "CLONING_COPY",
 *             description: Some(
 *                 "Create an exact duplicate of this object for use with the newly cloned scene.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "CLONING_BLANK_COPY",
 *             description: Some(
 *                 "Create a copy of this object for use with the newly cloned stream, leaving some of the data to be written later.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Sets how objects are to be cloned.
#[binrw::binrw]
pub enum CloningBehavior {
    #[brw(magic = 0_u32)]
    CloningShare,
    #[brw(magic = 1_u32)]
    CloningCopy,
    #[brw(magic = 2_u32)]
    CloningBlankCopy,
}
/*
 * Enum {
 *     name: "ComponentFormat",
 *     storage: "uint",
 *     description: Some(
 *         "The data format of components. Mask 0x00FF0000 to get the number of subfields. Mask 0x0000FF00 to get the size of each subfield.\r\n        It's not a bitfield, because the values are not independent.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0x00000000",
 *             name: "F_UNKNOWN",
 *             description: Some(
 *                 "Unknown, or don't care, format.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "0x00010101",
 *             name: "F_INT8_1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00020102",
 *             name: "F_INT8_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00030103",
 *             name: "F_INT8_3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00040104",
 *             name: "F_INT8_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00010105",
 *             name: "F_UINT8_1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00020106",
 *             name: "F_UINT8_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00030107",
 *             name: "F_UINT8_3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00040108",
 *             name: "F_UINT8_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00010109",
 *             name: "F_NORMINT8_1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0002010A",
 *             name: "F_NORMINT8_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0003010B",
 *             name: "F_NORMINT8_3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0004010C",
 *             name: "F_NORMINT8_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0001010D",
 *             name: "F_NORMUINT8_1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0002010E",
 *             name: "F_NORMUINT8_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0003010F",
 *             name: "F_NORMUINT8_3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00040110",
 *             name: "F_NORMUINT8_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00010211",
 *             name: "F_INT16_1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00020212",
 *             name: "F_INT16_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00030213",
 *             name: "F_INT16_3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00040214",
 *             name: "F_INT16_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00010215",
 *             name: "F_UINT16_1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00020216",
 *             name: "F_UINT16_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00030217",
 *             name: "F_UINT16_3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00040218",
 *             name: "F_UINT16_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00010219",
 *             name: "F_NORMINT16_1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0002021A",
 *             name: "F_NORMINT16_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0003021B",
 *             name: "F_NORMINT16_3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0004021C",
 *             name: "F_NORMINT16_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0001021D",
 *             name: "F_NORMUINT16_1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0002021E",
 *             name: "F_NORMUINT16_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0003021F",
 *             name: "F_NORMUINT16_3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00040220",
 *             name: "F_NORMUINT16_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00010421",
 *             name: "F_INT32_1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00020422",
 *             name: "F_INT32_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00030423",
 *             name: "F_INT32_3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00040424",
 *             name: "F_INT32_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00010425",
 *             name: "F_UINT32_1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00020426",
 *             name: "F_UINT32_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00030427",
 *             name: "F_UINT32_3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00040428",
 *             name: "F_UINT32_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00010429",
 *             name: "F_NORMINT32_1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0002042A",
 *             name: "F_NORMINT32_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0003042B",
 *             name: "F_NORMINT32_3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0004042C",
 *             name: "F_NORMINT32_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0001042D",
 *             name: "F_NORMUINT32_1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0002042E",
 *             name: "F_NORMUINT32_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0003042F",
 *             name: "F_NORMUINT32_3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00040430",
 *             name: "F_NORMUINT32_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00010231",
 *             name: "F_FLOAT16_1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00020232",
 *             name: "F_FLOAT16_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00030233",
 *             name: "F_FLOAT16_3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00040234",
 *             name: "F_FLOAT16_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00010435",
 *             name: "F_FLOAT32_1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00020436",
 *             name: "F_FLOAT32_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00030437",
 *             name: "F_FLOAT32_3",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00040438",
 *             name: "F_FLOAT32_4",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00010439",
 *             name: "F_UINT_10_10_10_L1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0001043A",
 *             name: "F_NORMINT_10_10_10_L1",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0001043B",
 *             name: "F_NORMINT_11_11_10",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0004013C",
 *             name: "F_NORMUINT8_4_BGRA",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0001043D",
 *             name: "F_NORMINT_10_10_10_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x0001043E",
 *             name: "F_UINT_10_10_10_2",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "0x00020240",
 *             name: "F_UNKNOWN_20240",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// The data format of components. Mask 0x00FF0000 to get the number of subfields. Mask 0x0000FF00 to get the size of each subfield.
/// It's not a bitfield, because the values are not independent.
#[binrw::binrw]
pub enum ComponentFormat {
    #[brw(magic = 0x00000000_u32)]
    FUnknown,
    #[brw(magic = 0x00010101_u32)]
    FInt81,
    #[brw(magic = 0x00020102_u32)]
    FInt82,
    #[brw(magic = 0x00030103_u32)]
    FInt83,
    #[brw(magic = 0x00040104_u32)]
    FInt84,
    #[brw(magic = 0x00010105_u32)]
    FUint81,
    #[brw(magic = 0x00020106_u32)]
    FUint82,
    #[brw(magic = 0x00030107_u32)]
    FUint83,
    #[brw(magic = 0x00040108_u32)]
    FUint84,
    #[brw(magic = 0x00010109_u32)]
    FNormint81,
    #[brw(magic = 0x0002010A_u32)]
    FNormint82,
    #[brw(magic = 0x0003010B_u32)]
    FNormint83,
    #[brw(magic = 0x0004010C_u32)]
    FNormint84,
    #[brw(magic = 0x0001010D_u32)]
    FNormuint81,
    #[brw(magic = 0x0002010E_u32)]
    FNormuint82,
    #[brw(magic = 0x0003010F_u32)]
    FNormuint83,
    #[brw(magic = 0x00040110_u32)]
    FNormuint84,
    #[brw(magic = 0x00010211_u32)]
    FInt161,
    #[brw(magic = 0x00020212_u32)]
    FInt162,
    #[brw(magic = 0x00030213_u32)]
    FInt163,
    #[brw(magic = 0x00040214_u32)]
    FInt164,
    #[brw(magic = 0x00010215_u32)]
    FUint161,
    #[brw(magic = 0x00020216_u32)]
    FUint162,
    #[brw(magic = 0x00030217_u32)]
    FUint163,
    #[brw(magic = 0x00040218_u32)]
    FUint164,
    #[brw(magic = 0x00010219_u32)]
    FNormint161,
    #[brw(magic = 0x0002021A_u32)]
    FNormint162,
    #[brw(magic = 0x0003021B_u32)]
    FNormint163,
    #[brw(magic = 0x0004021C_u32)]
    FNormint164,
    #[brw(magic = 0x0001021D_u32)]
    FNormuint161,
    #[brw(magic = 0x0002021E_u32)]
    FNormuint162,
    #[brw(magic = 0x0003021F_u32)]
    FNormuint163,
    #[brw(magic = 0x00040220_u32)]
    FNormuint164,
    #[brw(magic = 0x00010421_u32)]
    FInt321,
    #[brw(magic = 0x00020422_u32)]
    FInt322,
    #[brw(magic = 0x00030423_u32)]
    FInt323,
    #[brw(magic = 0x00040424_u32)]
    FInt324,
    #[brw(magic = 0x00010425_u32)]
    FUint321,
    #[brw(magic = 0x00020426_u32)]
    FUint322,
    #[brw(magic = 0x00030427_u32)]
    FUint323,
    #[brw(magic = 0x00040428_u32)]
    FUint324,
    #[brw(magic = 0x00010429_u32)]
    FNormint321,
    #[brw(magic = 0x0002042A_u32)]
    FNormint322,
    #[brw(magic = 0x0003042B_u32)]
    FNormint323,
    #[brw(magic = 0x0004042C_u32)]
    FNormint324,
    #[brw(magic = 0x0001042D_u32)]
    FNormuint321,
    #[brw(magic = 0x0002042E_u32)]
    FNormuint322,
    #[brw(magic = 0x0003042F_u32)]
    FNormuint323,
    #[brw(magic = 0x00040430_u32)]
    FNormuint324,
    #[brw(magic = 0x00010231_u32)]
    FFloat161,
    #[brw(magic = 0x00020232_u32)]
    FFloat162,
    #[brw(magic = 0x00030233_u32)]
    FFloat163,
    #[brw(magic = 0x00040234_u32)]
    FFloat164,
    #[brw(magic = 0x00010435_u32)]
    FFloat321,
    #[brw(magic = 0x00020436_u32)]
    FFloat322,
    #[brw(magic = 0x00030437_u32)]
    FFloat323,
    #[brw(magic = 0x00040438_u32)]
    FFloat324,
    #[brw(magic = 0x00010439_u32)]
    FUint101010L1,
    #[brw(magic = 0x0001043A_u32)]
    FNormint101010L1,
    #[brw(magic = 0x0001043B_u32)]
    FNormint111110,
    #[brw(magic = 0x0004013C_u32)]
    FNormuint84Bgra,
    #[brw(magic = 0x0001043D_u32)]
    FNormint1010102,
    #[brw(magic = 0x0001043E_u32)]
    FUint1010102,
    #[brw(magic = 0x00020240_u32)]
    FUnknown20240,
}
/*
 * Enum {
 *     name: "DataStreamUsage",
 *     storage: "uint",
 *     description: Some(
 *         "Determines how a data stream is used?",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "USAGE_VERTEX_INDEX",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "USAGE_VERTEX",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "USAGE_SHADER_CONSTANT",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "USAGE_USER",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "USAGE_UNKNOWN",
 *             description: Some(
 *                 "Seems to be associated with DISPLAYLIST component semantics.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Determines how a data stream is used?
#[binrw::binrw]
pub enum DataStreamUsage {
    #[brw(magic = 0_u32)]
    UsageVertexIndex,
    #[brw(magic = 1_u32)]
    UsageVertex,
    #[brw(magic = 2_u32)]
    UsageShaderConstant,
    #[brw(magic = 3_u32)]
    UsageUser,
    #[brw(magic = 4_u32)]
    UsageUnknown,
}
/*
 * Enum {
 *     name: "MeshPrimitiveType",
 *     storage: "uint",
 *     description: Some(
 *         "Describes the type of primitives stored in a mesh object.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "MESH_PRIMITIVE_TRIANGLES",
 *             description: Some(
 *                 "Triangle primitive type.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "MESH_PRIMITIVE_TRISTRIPS",
 *             description: Some(
 *                 "Triangle strip primitive type.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "MESH_PRIMITIVE_LINES",
 *             description: Some(
 *                 "Lines primitive type.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "MESH_PRIMITIVE_LINESTRIPS",
 *             description: Some(
 *                 "Line strip primitive type.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "MESH_PRIMITIVE_QUADS",
 *             description: Some(
 *                 "Quadrilateral primitive type.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "MESH_PRIMITIVE_POINTS",
 *             description: Some(
 *                 "Point primitive type.",
 *             ),
 *         },
 *     ],
 * }
 */
/// Describes the type of primitives stored in a mesh object.
#[binrw::binrw]
pub enum MeshPrimitiveType {
    #[brw(magic = 0_u32)]
    MeshPrimitiveTriangles,
    #[brw(magic = 1_u32)]
    MeshPrimitiveTristrips,
    #[brw(magic = 2_u32)]
    MeshPrimitiveLines,
    #[brw(magic = 3_u32)]
    MeshPrimitiveLinestrips,
    #[brw(magic = 4_u32)]
    MeshPrimitiveQuads,
    #[brw(magic = 5_u32)]
    MeshPrimitivePoints,
}
/*
 * Enum {
 *     name: "SyncPoint",
 *     storage: "ushort",
 *     description: Some(
 *         "A sync point corresponds to a particular stage in per-frame processing.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0x8000",
 *             name: "SYNC_ANY",
 *             description: Some(
 *                 "Synchronize for any sync points that the modifier supports.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "0x8010",
 *             name: "SYNC_UPDATE",
 *             description: Some(
 *                 "Synchronize when an object is updated.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "0x8020",
 *             name: "SYNC_POST_UPDATE",
 *             description: Some(
 *                 "Synchronize when an entire scene graph has been updated.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "0x8030",
 *             name: "SYNC_VISIBLE",
 *             description: Some(
 *                 "Synchronize when an object is determined to be potentially visible.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "0x8040",
 *             name: "SYNC_RENDER",
 *             description: Some(
 *                 "Synchronize when an object is rendered.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "0x8050",
 *             name: "SYNC_PHYSICS_SIMULATE",
 *             description: Some(
 *                 "Synchronize when a physics simulation step is about to begin.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "0x8060",
 *             name: "SYNC_PHYSICS_COMPLETED",
 *             description: Some(
 *                 "Synchronize when a physics simulation step has produced results.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "0x8070",
 *             name: "SYNC_REFLECTIONS",
 *             description: Some(
 *                 "Synchronize after all data necessary to calculate reflections is ready.",
 *             ),
 *         },
 *     ],
 * }
 */
/// A sync point corresponds to a particular stage in per-frame processing.
#[binrw::binrw]
pub enum SyncPoint {
    #[brw(magic = 0x8000_u16)]
    SyncAny,
    #[brw(magic = 0x8010_u16)]
    SyncUpdate,
    #[brw(magic = 0x8020_u16)]
    SyncPostUpdate,
    #[brw(magic = 0x8030_u16)]
    SyncVisible,
    #[brw(magic = 0x8040_u16)]
    SyncRender,
    #[brw(magic = 0x8050_u16)]
    SyncPhysicsSimulate,
    #[brw(magic = 0x8060_u16)]
    SyncPhysicsCompleted,
    #[brw(magic = 0x8070_u16)]
    SyncReflections,
}
/*
 * Enum {
 *     name: "AlignMethod",
 *     storage: "uint",
 *     description: Some(
 *         "Describes the various methods that may be used to specify the orientation of the particles.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "ALIGN_INVALID",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "ALIGN_PER_PARTICLE",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "ALIGN_LOCAL_FIXED",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "ALIGN_LOCAL_POSITION",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "9",
 *             name: "ALIGN_LOCAL_VELOCITY",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "16",
 *             name: "ALIGN_CAMERA",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// Describes the various methods that may be used to specify the orientation of the particles.
#[binrw::binrw]
pub enum AlignMethod {
    #[brw(magic = 0_u32)]
    AlignInvalid,
    #[brw(magic = 1_u32)]
    AlignPerParticle,
    #[brw(magic = 2_u32)]
    AlignLocalFixed,
    #[brw(magic = 5_u32)]
    AlignLocalPosition,
    #[brw(magic = 9_u32)]
    AlignLocalVelocity,
    #[brw(magic = 16_u32)]
    AlignCamera,
}
/*
 * Enum {
 *     name: "PSLoopBehavior",
 *     storage: "uint",
 *     description: None,
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "PS_LOOP_CLAMP_BIRTH",
 *             description: Some(
 *                 "Key times map such that the first key occurs at the birth of the particle, and times later than the last key get the last key value.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "PS_LOOP_CLAMP_DEATH",
 *             description: Some(
 *                 "Key times map such that the last key occurs at the death of the particle, and times before the initial key time get the value of the initial key.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "PS_LOOP_AGESCALE",
 *             description: Some(
 *                 "Scale the animation to fit the particle lifetime, so that the first key is age zero, and the last key comes at the particle death.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "PS_LOOP_LOOP",
 *             description: Some(
 *                 "The time is converted to one within the time range represented by the keys, as if the key sequence loops forever in the past and future.",
 *             ),
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "PS_LOOP_REFLECT",
 *             description: Some(
 *                 "The time is reflection looped, as if the keys played forward then backward the forward then backward etc for all time.",
 *             ),
 *         },
 *     ],
 * }
 */
#[binrw::binrw]
pub enum PSLoopBehavior {
    #[brw(magic = 0_u32)]
    PsLoopClampBirth,
    #[brw(magic = 1_u32)]
    PsLoopClampDeath,
    #[brw(magic = 2_u32)]
    PsLoopAgescale,
    #[brw(magic = 3_u32)]
    PsLoopLoop,
    #[brw(magic = 4_u32)]
    PsLoopReflect,
}
/*
 * Enum {
 *     name: "PSForceType",
 *     storage: "uint",
 *     description: Some(
 *         "This is used by the Floodgate kernel to determine which NiPSForceHelpers functions to call.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "FORCE_BOMB",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "FORCE_DRAG",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "2",
 *             name: "FORCE_AIR_FIELD",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "3",
 *             name: "FORCE_DRAG_FIELD",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "4",
 *             name: "FORCE_GRAVITY_FIELD",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "5",
 *             name: "FORCE_RADIAL_FIELD",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "6",
 *             name: "FORCE_TURBULENCE_FIELD",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "7",
 *             name: "FORCE_VORTEX_FIELD",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "8",
 *             name: "FORCE_GRAVITY",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// This is used by the Floodgate kernel to determine which NiPSForceHelpers functions to call.
#[binrw::binrw]
pub enum PSForceType {
    #[brw(magic = 0_u32)]
    ForceBomb,
    #[brw(magic = 1_u32)]
    ForceDrag,
    #[brw(magic = 2_u32)]
    ForceAirField,
    #[brw(magic = 3_u32)]
    ForceDragField,
    #[brw(magic = 4_u32)]
    ForceGravityField,
    #[brw(magic = 5_u32)]
    ForceRadialField,
    #[brw(magic = 6_u32)]
    ForceTurbulenceField,
    #[brw(magic = 7_u32)]
    ForceVortexField,
    #[brw(magic = 8_u32)]
    ForceGravity,
}
/*
 * Enum {
 *     name: "ColliderType",
 *     storage: "uint",
 *     description: Some(
 *         "This is used by the Floodgate kernel to determine which NiPSColliderHelpers functions to call.",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "COLLIDER_PLANAR",
 *             description: None,
 *         },
 *         EnumOption {
 *             value: "1",
 *             name: "COLLIDER_SPHERICAL",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// This is used by the Floodgate kernel to determine which NiPSColliderHelpers functions to call.
#[binrw::binrw]
pub enum ColliderType {
    #[brw(magic = 0_u32)]
    ColliderPlanar,
    #[brw(magic = 1_u32)]
    ColliderSpherical,
}
/*
 * Enum {
 *     name: "hkWeldingType",
 *     storage: "byte",
 *     description: Some(
 *         "hkpWeldingUtility::WeldingType",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "0",
 *             name: "ANTICLOCKWISE",
 *             description: None,
 *         },
 *     ],
 * }
 */
/// hkpWeldingUtility::WeldingType
#[binrw::binrw]
pub enum hkWeldingType {
    #[brw(magic = 0_u8)]
    Anticlockwise,
}
/*
 * Enum {
 *     name: "bhkCMSMatType",
 *     storage: "byte",
 *     description: Some(
 *         "hkpCompressedMeshShape::MaterialType",
 *     ),
 *     options: [
 *         EnumOption {
 *             value: "1",
 *             name: "SINGLE_VALUE_PER_CHUNK",
 *             description: Some(
 *                 "Chunk builder makes sure that only chunks with the same material are created.",
 *             ),
 *         },
 *     ],
 * }
 */
/// hkpCompressedMeshShape::MaterialType
#[binrw::binrw]
pub enum bhkCMSMatType {
    #[brw(magic = 1_u8)]
    SingleValuePerChunk,
}
mod NiMain {
    use super::*;
    use crate::basic_types::*;
    /*
     * Struct {
     *     name: "SizedString",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Length",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The string length.",
     *             ),
     *         },
     *         StructField {
     *             name: "Value",
     *             type: "char",
     *             length: Some(
     *                 "Length",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The string itself.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A string of given length.",
     *     ),
     * }
     */
    /// A string of given length.
    #[binrw::binrw]
    pub struct SizedString {
        /// The string length.
        pub length: u32,
        /// The string itself.
        #[br(count = length)]
        pub value: Vec<u8>,
    }
    /*
     * Struct {
     *     name: "SizedString16",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Length",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The string length.",
     *             ),
     *         },
     *         StructField {
     *             name: "Value",
     *             type: "char",
     *             length: Some(
     *                 "Length",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The string itself.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A string of given length, using a ushort to store string length.",
     *     ),
     * }
     */
    /// A string of given length, using a ushort to store string length.
    #[binrw::binrw]
    pub struct SizedString16 {
        /// The string length.
        pub length: u16,
        /// The string itself.
        #[br(count = length)]
        pub value: Vec<u8>,
    }
    /*
     * Struct {
     *     name: "string",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "String",
     *             type: "SizedString",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The normal string.",
     *             ),
     *         },
     *         StructField {
     *             name: "Index",
     *             type: "NiFixedString",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "20.1.0.3",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "The string index.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A string type.",
     *     ),
     * }
     */
    /// A string type.
    #[binrw::binrw]
    pub struct string {
        /// The normal string.
        pub string: SizedString,
        // field is out of version range (Some("20.1.0.3"), None)
        // pub index: NiFixedString,
    }
    /*
     * Struct {
     *     name: "NiTFixedStringMapItem",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: Some(
     *         "true",
     *     ),
     *     fields: [
     *         StructField {
     *             name: "String",
     *             type: "NiFixedString",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Value",
     *             type: "#T#",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: Some(
     *         "Currently, #T# must be a basic type due to nif.xml restrictions.",
     *     ),
     * }
     */
    /// Currently, #T# must be a basic type due to nif.xml restrictions.
    #[binrw::binrw]
    pub struct NiTFixedStringMapItem<T: binrw::BinRead + binrw::BinWrite + 'static>
    where
        T: for<'a> binrw::BinRead<Args<'a> = ()>,
        T: for<'a> binrw::BinWrite<Args<'a> = ()>,
    {
        pub string: NiFixedString,
        pub value: T,
    }
    /*
     * Struct {
     *     name: "NiTFixedStringMap",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: Some(
     *         "true",
     *     ),
     *     fields: [
     *         StructField {
     *             name: "Num Strings",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Strings",
     *             type: "NiTFixedStringMapItem",
     *             length: Some(
     *                 "Num Strings",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: Some(
     *                 "#T#",
     *             ),
     *             description: None,
     *         },
     *     ],
     *     description: Some(
     *         "A mapping or hash table between NiFixedString keys and a generic value.\r\n        Currently, #T# must be a basic type due to nif.xml restrictions.",
     *     ),
     * }
     */
    /// A mapping or hash table between NiFixedString keys and a generic value.
    /// Currently, #T# must be a basic type due to nif.xml restrictions.
    #[binrw::binrw]
    pub struct NiTFixedStringMap<T: binrw::BinRead + binrw::BinWrite + 'static>
    where
        T: for<'a> binrw::BinRead<Args<'a> = ()>,
        T: for<'a> binrw::BinWrite<Args<'a> = ()>,
    {
        pub num_strings: u32,
        #[br(count = num_strings)]
        pub strings: Vec<NiTFixedStringMapItem<T>>,
    }
    /*
     * Struct {
     *     name: "ByteArray",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Data Size",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The number of bytes in this array",
     *             ),
     *         },
     *         StructField {
     *             name: "Data",
     *             type: "byte",
     *             length: Some(
     *                 "Data Size",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The bytes which make up the array",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "An array of bytes.",
     *     ),
     * }
     */
    /// An array of bytes.
    #[binrw::binrw]
    pub struct ByteArray {
        /// The number of bytes in this array
        pub data_size: u32,
        /// The bytes which make up the array
        #[br(count = data_size)]
        pub data: Vec<u8>,
    }
    /*
     * Struct {
     *     name: "ByteMatrix",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Data Size 1",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The number of bytes in this array",
     *             ),
     *         },
     *         StructField {
     *             name: "Data Size 2",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The number of bytes in this array",
     *             ),
     *         },
     *         StructField {
     *             name: "Data",
     *             type: "byte",
     *             length: Some(
     *                 "Data Size 2",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The bytes which make up the array",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "An array of bytes.",
     *     ),
     * }
     */
    /// An array of bytes.
    #[binrw::binrw]
    pub struct ByteMatrix {
        /// The number of bytes in this array
        pub data_size_1: u32,
        /// The number of bytes in this array
        pub data_size_2: u32,
        /// The bytes which make up the array
        #[br(count = data_size_2)]
        pub data: Vec<u8>,
    }
    /*
     * Struct {
     *     name: "Color3",
     *     size: Some(
     *         12,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "r",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Red color component.",
     *             ),
     *         },
     *         StructField {
     *             name: "g",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Green color component.",
     *             ),
     *         },
     *         StructField {
     *             name: "b",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Blue color component.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A color without alpha (red, green, blue).",
     *     ),
     * }
     */
    /// A color without alpha (red, green, blue).
    #[binrw::binrw]
    pub struct Color3 {
        /// Red color component.
        pub r: f32,
        /// Green color component.
        pub g: f32,
        /// Blue color component.
        pub b: f32,
    }
    /*
     * Struct {
     *     name: "ByteColor3",
     *     size: Some(
     *         3,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "r",
     *             type: "byte",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Red color component.",
     *             ),
     *         },
     *         StructField {
     *             name: "g",
     *             type: "byte",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Green color component.",
     *             ),
     *         },
     *         StructField {
     *             name: "b",
     *             type: "byte",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Blue color component.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A color without alpha (red, green, blue).",
     *     ),
     * }
     */
    /// A color without alpha (red, green, blue).
    #[binrw::binrw]
    pub struct ByteColor3 {
        /// Red color component.
        pub r: u8,
        /// Green color component.
        pub g: u8,
        /// Blue color component.
        pub b: u8,
    }
    /*
     * Struct {
     *     name: "Color4",
     *     size: Some(
     *         16,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "r",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Red component.",
     *             ),
     *         },
     *         StructField {
     *             name: "g",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Green component.",
     *             ),
     *         },
     *         StructField {
     *             name: "b",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Blue component.",
     *             ),
     *         },
     *         StructField {
     *             name: "a",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Alpha.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A color with alpha (red, green, blue, alpha).",
     *     ),
     * }
     */
    /// A color with alpha (red, green, blue, alpha).
    #[binrw::binrw]
    pub struct Color4 {
        /// Red component.
        pub r: f32,
        /// Green component.
        pub g: f32,
        /// Blue component.
        pub b: f32,
        /// Alpha.
        pub a: f32,
    }
    /*
     * Struct {
     *     name: "ByteColor4",
     *     size: Some(
     *         4,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "r",
     *             type: "byte",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Red color component.",
     *             ),
     *         },
     *         StructField {
     *             name: "g",
     *             type: "byte",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Green color component.",
     *             ),
     *         },
     *         StructField {
     *             name: "b",
     *             type: "byte",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Blue color component.",
     *             ),
     *         },
     *         StructField {
     *             name: "a",
     *             type: "byte",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Alpha color component.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A color with alpha (red, green, blue, alpha).",
     *     ),
     * }
     */
    /// A color with alpha (red, green, blue, alpha).
    #[binrw::binrw]
    pub struct ByteColor4 {
        /// Red color component.
        pub r: u8,
        /// Green color component.
        pub g: u8,
        /// Blue color component.
        pub b: u8,
        /// Alpha color component.
        pub a: u8,
    }
    /*
     * Struct {
     *     name: "FilePath",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "String",
     *             type: "SizedString",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The normal string.",
     *             ),
     *         },
     *         StructField {
     *             name: "Index",
     *             type: "NiFixedString",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "20.1.0.3",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "The string index.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A string that contains the path to a file.",
     *     ),
     * }
     */
    /// A string that contains the path to a file.
    #[binrw::binrw]
    pub struct FilePath {
        /// The normal string.
        pub string: SizedString,
        // field is out of version range (Some("20.1.0.3"), None)
        // pub index: NiFixedString,
    }
    /*
     * Struct {
     *     name: "Footer",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Num Roots",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "3.3.0.13",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "The number of root references.",
     *             ),
     *         },
     *         StructField {
     *             name: "Roots",
     *             type: "Ref",
     *             length: Some(
     *                 "Num Roots",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "3.3.0.13",
     *             ),
     *             template: Some(
     *                 "NiObject",
     *             ),
     *             description: Some(
     *                 "List of root NIF objects. If there is a camera, for 1st person view, then this NIF object is referred to as well in this list, even if it is not a root object (usually we want the camera to be attached to the Bip Head node).",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "The NIF file footer.",
     *     ),
     * }
     */
    /// The NIF file footer.
    #[binrw::binrw]
    pub struct Footer {
        /// The number of root references.
        pub num_roots: u32,
        /// List of root NIF objects. If there is a camera, for 1st person view, then this NIF object is referred to as well in this list, even if it is not a root object (usually we want the camera to be attached to the Bip Head node).
        #[br(count = num_roots)]
        pub roots: Vec<Ref<NiObject>>,
    }
    /*
     * Struct {
     *     name: "LODRange",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Near Extent",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Beginning of range.",
     *             ),
     *         },
     *         StructField {
     *             name: "Far Extent",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "End of Range.",
     *             ),
     *         },
     *         StructField {
     *             name: "Unknown Ints",
     *             type: "uint",
     *             length: Some(
     *                 "3",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "3.1",
     *             ),
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: Some(
     *         "The distance range where a specific level of detail applies.",
     *     ),
     * }
     */
    /// The distance range where a specific level of detail applies.
    #[binrw::binrw]
    pub struct LODRange {
        /// Beginning of range.
        pub near_extent: f32,
        /// End of Range.
        pub far_extent: f32,
        // field is out of version range (None, Some("3.1"))
        // pub unknown_ints: Vec<u32>,
    }
    /*
     * Struct {
     *     name: "MatchGroup",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Num Vertices",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Number of vertices in this group.",
     *             ),
     *         },
     *         StructField {
     *             name: "Vertex Indices",
     *             type: "ushort",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The vertex indices.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "Group of vertex indices of vertices that match.",
     *     ),
     * }
     */
    /// Group of vertex indices of vertices that match.
    #[binrw::binrw]
    pub struct MatchGroup {
        /// Number of vertices in this group.
        pub num_vertices: u16,
        /// The vertex indices.
        #[br(count = num_vertices)]
        pub vertex_indices: Vec<u16>,
    }
    /*
     * Struct {
     *     name: "Vector3",
     *     size: Some(
     *         12,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "x",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "First coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "y",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Second coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "z",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Third coordinate.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A vector in 3D space (x,y,z).",
     *     ),
     * }
     */
    /// A vector in 3D space (x,y,z).
    #[binrw::binrw]
    pub struct Vector3 {
        /// First coordinate.
        pub x: f32,
        /// Second coordinate.
        pub y: f32,
        /// Third coordinate.
        pub z: f32,
    }
    /*
     * Struct {
     *     name: "HalfVector3",
     *     size: Some(
     *         6,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "x",
     *             type: "hfloat",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "First coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "y",
     *             type: "hfloat",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Second coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "z",
     *             type: "hfloat",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Third coordinate.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A vector in 3D space (x,y,z).",
     *     ),
     * }
     */
    /// A vector in 3D space (x,y,z).
    #[binrw::binrw]
    pub struct HalfVector3 {
        /// First coordinate.
        pub x: Float16,
        /// Second coordinate.
        pub y: Float16,
        /// Third coordinate.
        pub z: Float16,
    }
    /*
     * Struct {
     *     name: "UshortVector3",
     *     size: Some(
     *         6,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "x",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "First coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "y",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Second coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "z",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Third coordinate.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A vector in 3D space (x,y,z).",
     *     ),
     * }
     */
    /// A vector in 3D space (x,y,z).
    #[binrw::binrw]
    pub struct UshortVector3 {
        /// First coordinate.
        pub x: u16,
        /// Second coordinate.
        pub y: u16,
        /// Third coordinate.
        pub z: u16,
    }
    /*
     * Struct {
     *     name: "ByteVector3",
     *     size: Some(
     *         3,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "x",
     *             type: "normbyte",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "First coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "y",
     *             type: "normbyte",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Second coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "z",
     *             type: "normbyte",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Third coordinate.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A vector in 3D space (x,y,z).",
     *     ),
     * }
     */
    /// A vector in 3D space (x,y,z).
    #[binrw::binrw]
    pub struct ByteVector3 {
        /// First coordinate.
        pub x: NormByte,
        /// Second coordinate.
        pub y: NormByte,
        /// Third coordinate.
        pub z: NormByte,
    }
    /*
     * Struct {
     *     name: "Vector4",
     *     size: Some(
     *         16,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "x",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "First coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "y",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Second coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "z",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Third coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "w",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Fourth coordinate.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A 4-dimensional vector.",
     *     ),
     * }
     */
    /// A 4-dimensional vector.
    #[binrw::binrw]
    pub struct Vector4 {
        /// First coordinate.
        pub x: f32,
        /// Second coordinate.
        pub y: f32,
        /// Third coordinate.
        pub z: f32,
        /// Fourth coordinate.
        pub w: f32,
    }
    /*
     * Struct {
     *     name: "Quaternion",
     *     size: Some(
     *         16,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "w",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The w-coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "x",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The x-coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "y",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The y-coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "z",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The z-coordinate.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A quaternion.",
     *     ),
     * }
     */
    /// A quaternion.
    #[binrw::binrw]
    pub struct Quaternion {
        /// The w-coordinate.
        pub w: f32,
        /// The x-coordinate.
        pub x: f32,
        /// The y-coordinate.
        pub y: f32,
        /// The z-coordinate.
        pub z: f32,
    }
    /*
     * Struct {
     *     name: "Matrix22",
     *     size: Some(
     *         16,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "m11",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Member 1,1 (top left)",
     *             ),
     *         },
     *         StructField {
     *             name: "m21",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Member 2,1 (bottom left)",
     *             ),
     *         },
     *         StructField {
     *             name: "m12",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Member 1,2 (top right)",
     *             ),
     *         },
     *         StructField {
     *             name: "m22",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Member 2,2 (bottom right)",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A 2x2 matrix of float values.  Stored in OpenGL column-major format.",
     *     ),
     * }
     */
    /// A 2x2 matrix of float values.  Stored in OpenGL column-major format.
    #[binrw::binrw]
    pub struct Matrix22 {
        /// Member 1,1 (top left)
        pub m11: f32,
        /// Member 2,1 (bottom left)
        pub m21: f32,
        /// Member 1,2 (top right)
        pub m12: f32,
        /// Member 2,2 (bottom right)
        pub m22: f32,
    }
    /*
     * Struct {
     *     name: "Matrix33",
     *     size: Some(
     *         36,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "m11",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Member 1,1 (top left)",
     *             ),
     *         },
     *         StructField {
     *             name: "m21",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Member 2,1",
     *             ),
     *         },
     *         StructField {
     *             name: "m31",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Member 3,1 (bottom left)",
     *             ),
     *         },
     *         StructField {
     *             name: "m12",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Member 1,2",
     *             ),
     *         },
     *         StructField {
     *             name: "m22",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Member 2,2",
     *             ),
     *         },
     *         StructField {
     *             name: "m32",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Member 3,2",
     *             ),
     *         },
     *         StructField {
     *             name: "m13",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Member 1,3 (top right)",
     *             ),
     *         },
     *         StructField {
     *             name: "m23",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Member 2,3",
     *             ),
     *         },
     *         StructField {
     *             name: "m33",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Member 3,3 (bottom left)",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A 3x3 rotation matrix; M^T M=identity, det(M)=1.    Stored in OpenGL column-major format.",
     *     ),
     * }
     */
    /// A 3x3 rotation matrix; M^T M=identity, det(M)=1.    Stored in OpenGL column-major format.
    #[binrw::binrw]
    pub struct Matrix33 {
        /// Member 1,1 (top left)
        pub m11: f32,
        /// Member 2,1
        pub m21: f32,
        /// Member 3,1 (bottom left)
        pub m31: f32,
        /// Member 1,2
        pub m12: f32,
        /// Member 2,2
        pub m22: f32,
        /// Member 3,2
        pub m32: f32,
        /// Member 1,3 (top right)
        pub m13: f32,
        /// Member 2,3
        pub m23: f32,
        /// Member 3,3 (bottom left)
        pub m33: f32,
    }
    /*
     * Struct {
     *     name: "Matrix34",
     *     size: Some(
     *         48,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "m11",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (1,1) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m21",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (2,1) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m31",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (3,1) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m12",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (1,2) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m22",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (2,2) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m32",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (3,2) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m13",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (1,3) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m23",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (2,3) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m33",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (3,3) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m14",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (1,4) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m24",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (2,4) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m34",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (3,4) element.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A 3x4 transformation matrix.",
     *     ),
     * }
     */
    /// A 3x4 transformation matrix.
    #[binrw::binrw]
    pub struct Matrix34 {
        /// The (1,1) element.
        pub m11: f32,
        /// The (2,1) element.
        pub m21: f32,
        /// The (3,1) element.
        pub m31: f32,
        /// The (1,2) element.
        pub m12: f32,
        /// The (2,2) element.
        pub m22: f32,
        /// The (3,2) element.
        pub m32: f32,
        /// The (1,3) element.
        pub m13: f32,
        /// The (2,3) element.
        pub m23: f32,
        /// The (3,3) element.
        pub m33: f32,
        /// The (1,4) element.
        pub m14: f32,
        /// The (2,4) element.
        pub m24: f32,
        /// The (3,4) element.
        pub m34: f32,
    }
    /*
     * Struct {
     *     name: "Matrix44",
     *     size: Some(
     *         64,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "m11",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (1,1) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m21",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (2,1) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m31",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (3,1) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m41",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (4,1) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m12",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (1,2) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m22",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (2,2) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m32",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (3,2) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m42",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (4,2) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m13",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (1,3) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m23",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (2,3) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m33",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (3,3) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m43",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (4,3) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m14",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (1,4) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m24",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (2,4) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m34",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (3,4) element.",
     *             ),
     *         },
     *         StructField {
     *             name: "m44",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The (4,4) element.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A 4x4 transformation matrix.",
     *     ),
     * }
     */
    /// A 4x4 transformation matrix.
    #[binrw::binrw]
    pub struct Matrix44 {
        /// The (1,1) element.
        pub m11: f32,
        /// The (2,1) element.
        pub m21: f32,
        /// The (3,1) element.
        pub m31: f32,
        /// The (4,1) element.
        pub m41: f32,
        /// The (1,2) element.
        pub m12: f32,
        /// The (2,2) element.
        pub m22: f32,
        /// The (3,2) element.
        pub m32: f32,
        /// The (4,2) element.
        pub m42: f32,
        /// The (1,3) element.
        pub m13: f32,
        /// The (2,3) element.
        pub m23: f32,
        /// The (3,3) element.
        pub m33: f32,
        /// The (4,3) element.
        pub m43: f32,
        /// The (1,4) element.
        pub m14: f32,
        /// The (2,4) element.
        pub m24: f32,
        /// The (3,4) element.
        pub m34: f32,
        /// The (4,4) element.
        pub m44: f32,
    }
    /*
     * Struct {
     *     name: "MipMap",
     *     size: Some(
     *         12,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Width",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Width of the mipmap image.",
     *             ),
     *         },
     *         StructField {
     *             name: "Height",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Height of the mipmap image.",
     *             ),
     *         },
     *         StructField {
     *             name: "Offset",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Offset into the pixel data array where this mipmap starts.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "Description of a mipmap within an NiPixelData object.",
     *     ),
     * }
     */
    /// Description of a mipmap within an NiPixelData object.
    #[binrw::binrw]
    pub struct MipMap {
        /// Width of the mipmap image.
        pub width: u32,
        /// Height of the mipmap image.
        pub height: u32,
        /// Offset into the pixel data array where this mipmap starts.
        pub offset: u32,
    }
    /*
     * Struct {
     *     name: "BoneVertData",
     *     size: Some(
     *         6,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Index",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The vertex index, in the mesh.",
     *             ),
     *         },
     *         StructField {
     *             name: "Weight",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The vertex weight - between 0.0 and 1.0",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "NiSkinData::BoneVertData. A vertex and its weight.",
     *     ),
     * }
     */
    /// NiSkinData::BoneVertData. A vertex and its weight.
    #[binrw::binrw]
    pub struct BoneVertData {
        /// The vertex index, in the mesh.
        pub index: u16,
        /// The vertex weight - between 0.0 and 1.0
        pub weight: f32,
    }
    /*
     * Struct {
     *     name: "AVObject",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Name",
     *             type: "SizedString",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Object name.",
     *             ),
     *         },
     *         StructField {
     *             name: "AV Object",
     *             type: "Ptr",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: Some(
     *                 "NiAVObject",
     *             ),
     *             description: Some(
     *                 "Object reference.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "Used in NiDefaultAVObjectPalette.",
     *     ),
     * }
     */
    /// Used in NiDefaultAVObjectPalette.
    #[binrw::binrw]
    pub struct AVObject {
        /// Object name.
        pub name: SizedString,
        /// Object reference.
        pub av_object: Ptr<NiAVObject>,
    }
    /*
     * Struct {
     *     name: "Header",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Header String",
     *             type: "HeaderString",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "'NetImmerse File Format x.x.x.x' (versions <= 10.0.1.2) or 'Gamebryo File Format x.x.x.x' (versions >= 10.1.0.0), with x.x.x.x the version written out. Ends with a newline character (0x0A).",
     *             ),
     *         },
     *         StructField {
     *             name: "Copyright",
     *             type: "LineString",
     *             length: Some(
     *                 "3",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "3.1.0.0",
     *             ),
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Version",
     *             type: "FileVersion",
     *             length: None,
     *             default: Some(
     *                 "0x04000002",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "3.1.0.1",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "The NIF version, in hexadecimal notation: 0x04000002, 0x0401000C, 0x04020002, 0x04020100, 0x04020200, 0x0A000100, 0x0A010000, 0x0A020000, 0x14000004, ...",
     *             ),
     *         },
     *         StructField {
     *             name: "Endian Type",
     *             type: "EndianType",
     *             length: None,
     *             default: Some(
     *                 "ENDIAN_LITTLE",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "20.0.0.3",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Determines the endianness of the data in the file.",
     *             ),
     *         },
     *         StructField {
     *             name: "User Version",
     *             type: "ulittle32",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "10.0.1.8",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "An extra version number, for companies that decide to modify the file format.",
     *             ),
     *         },
     *         StructField {
     *             name: "Num Blocks",
     *             type: "ulittle32",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "3.1.0.1",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Number of file objects.",
     *             ),
     *         },
     *         StructField {
     *             name: "BS Header",
     *             type: "BSStreamHeader",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "#BSSTREAMHEADER#",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Metadata",
     *             type: "ByteArray",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "30.0.0.0",
     *             ),
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Num Block Types",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "5.0.0.1",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Number of object types in this NIF file.",
     *             ),
     *         },
     *         StructField {
     *             name: "Block Types",
     *             type: "SizedString",
     *             length: Some(
     *                 "Num Block Types",
     *             ),
     *             default: None,
     *             cond: Some(
     *                 "Version != 20.3.1.2",
     *             ),
     *             until: None,
     *             since: Some(
     *                 "5.0.0.1",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "List of all object types used in this NIF file.",
     *             ),
     *         },
     *         StructField {
     *             name: "Block Type Hashes",
     *             type: "uint",
     *             length: Some(
     *                 "Num Block Types",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "20.3.1.2",
     *             ),
     *             since: Some(
     *                 "20.3.1.2",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "List of all object types used in this NIF file.",
     *             ),
     *         },
     *         StructField {
     *             name: "Block Type Index",
     *             type: "BlockTypeIndex",
     *             length: Some(
     *                 "Num Blocks",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "5.0.0.1",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Maps file objects on their corresponding type: first file object is of type object_types[object_type_index[0]], the second of object_types[object_type_index[1]], etc.",
     *             ),
     *         },
     *         StructField {
     *             name: "Block Size",
     *             type: "uint",
     *             length: Some(
     *                 "Num Blocks",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "20.2.0.5",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Array of block sizes",
     *             ),
     *         },
     *         StructField {
     *             name: "Num Strings",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "20.1.0.1",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Number of strings.",
     *             ),
     *         },
     *         StructField {
     *             name: "Max String Length",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "20.1.0.1",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Maximum string length.",
     *             ),
     *         },
     *         StructField {
     *             name: "Strings",
     *             type: "SizedString",
     *             length: Some(
     *                 "Num Strings",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "20.1.0.1",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Strings.",
     *             ),
     *         },
     *         StructField {
     *             name: "Num Groups",
     *             type: "uint",
     *             length: None,
     *             default: Some(
     *                 "0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "5.0.0.6",
     *             ),
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Groups",
     *             type: "uint",
     *             length: Some(
     *                 "Num Groups",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "5.0.0.6",
     *             ),
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: Some(
     *         "The NIF file header.",
     *     ),
     * }
     */
    /// The NIF file header.
    #[binrw::binrw]
    pub struct Header {
        /// 'NetImmerse File Format x.x.x.x' (versions <= 10.0.1.2) or 'Gamebryo File Format x.x.x.x' (versions >= 10.1.0.0), with x.x.x.x the version written out. Ends with a newline character (0x0A).
        pub header_string: HeaderString,
        // field is out of version range (None, Some("3.1.0.0"))
        // pub copyright: Vec<LineString>,
        /// The NIF version, in hexadecimal notation: 0x04000002, 0x0401000C, 0x04020002, 0x04020100, 0x04020200, 0x0A000100, 0x0A010000, 0x0A020000, 0x14000004, ...
        pub version: FileVersion,
        /// Determines the endianness of the data in the file.
        pub endian_type: EndianType,
        /// An extra version number, for companies that decide to modify the file format.
        pub user_version: u32,
        /// Number of file objects.
        pub num_blocks: u32,
        pub bs_header: BSStreamHeader,
        // field is out of version range (Some("30.0.0.0"), None)
        // pub metadata: ByteArray,
        /// Number of object types in this NIF file.
        pub num_block_types: u16,
        /// List of all object types used in this NIF file.
        #[br(count = num_block_types)]
        pub block_types: Vec<SizedString>,
        // field is out of version range (Some("20.3.1.2"), Some("20.3.1.2"))
        // pub block_type_hashes: Vec<u32>,
        /// Maps file objects on their corresponding type: first file object is of type object_types[object_type_index[0]], the second of object_types[object_type_index[1]], etc.
        #[br(count = num_blocks)]
        pub block_type_index: Vec<BlockTypeIndex>,
        // field is out of version range (Some("20.2.0.5"), None)
        // pub block_size: Vec<u32>,
        // field is out of version range (Some("20.1.0.1"), None)
        // pub num_strings: u32,
        // field is out of version range (Some("20.1.0.1"), None)
        // pub max_string_length: u32,
        // field is out of version range (Some("20.1.0.1"), None)
        // pub strings: Vec<SizedString>,
        pub num_groups: u32,
        #[br(count = num_groups)]
        pub groups: Vec<u32>,
    }
    /*
     * Struct {
     *     name: "StringPalette",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Palette",
     *             type: "SizedString",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "A bunch of 0x00 seperated strings.",
     *             ),
     *         },
     *         StructField {
     *             name: "Length",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Length of the palette string is repeated here.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A list of \\\\0 terminated strings.",
     *     ),
     * }
     */
    /// A list of \\0 terminated strings.
    #[binrw::binrw]
    pub struct StringPalette {
        /// A bunch of 0x00 seperated strings.
        pub palette: SizedString,
        /// Length of the palette string is repeated here.
        pub length: u32,
    }
    /*
     * Struct {
     *     name: "TBC",
     *     size: Some(
     *         12,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "t",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Tension.",
     *             ),
     *         },
     *         StructField {
     *             name: "b",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Bias.",
     *             ),
     *         },
     *         StructField {
     *             name: "c",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Continuity.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "Tension, bias, continuity.",
     *     ),
     * }
     */
    /// Tension, bias, continuity.
    #[binrw::binrw]
    pub struct TBC {
        /// Tension.
        pub t: f32,
        /// Bias.
        pub b: f32,
        /// Continuity.
        pub c: f32,
    }
    /*
     * Struct {
     *     name: "Key",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: Some(
     *         "true",
     *     ),
     *     fields: [
     *         StructField {
     *             name: "Time",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Time of the key.",
     *             ),
     *         },
     *         StructField {
     *             name: "Value",
     *             type: "#T#",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The key value.",
     *             ),
     *         },
     *         StructField {
     *             name: "Forward",
     *             type: "#T#",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "#ARG# == 2",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Key forward tangent.",
     *             ),
     *         },
     *         StructField {
     *             name: "Backward",
     *             type: "#T#",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "#ARG# == 2",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The key backward tangent.",
     *             ),
     *         },
     *         StructField {
     *             name: "TBC",
     *             type: "TBC",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "#ARG# == 3",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The TBC of the key.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A generic key with support for interpolation. Type 1 is normal linear interpolation, type 2 has forward and backward tangents, and type 3 has tension, bias and continuity arguments. Note that color4 and byte always seem to be of type 1.",
     *     ),
     * }
     */
    /// A generic key with support for interpolation. Type 1 is normal linear interpolation, type 2 has forward and backward tangents, and type 3 has tension, bias and continuity arguments. Note that color4 and byte always seem to be of type 1.
    #[binrw::binrw]
    pub struct Key<T: binrw::BinRead + binrw::BinWrite + 'static>
    where
        T: for<'a> binrw::BinRead<Args<'a> = ()>,
        T: for<'a> binrw::BinWrite<Args<'a> = ()>,
    {
        /// Time of the key.
        pub time: f32,
        /// The key value.
        pub value: T,
        /// Key forward tangent.
        pub forward: T,
        /// The key backward tangent.
        pub backward: T,
        /// The TBC of the key.
        pub tbc: TBC,
    }
    /*
     * Struct {
     *     name: "KeyGroup",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: Some(
     *         "true",
     *     ),
     *     fields: [
     *         StructField {
     *             name: "Num Keys",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Number of keys in the array.",
     *             ),
     *         },
     *         StructField {
     *             name: "Interpolation",
     *             type: "KeyType",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "Num Keys != 0",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The key type.",
     *             ),
     *         },
     *         StructField {
     *             name: "Keys",
     *             type: "Key",
     *             length: Some(
     *                 "Num Keys",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: Some(
     *                 "#T#",
     *             ),
     *             description: Some(
     *                 "The keys.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "Array of vector keys (anything that can be interpolated, except rotations).",
     *     ),
     * }
     */
    /// Array of vector keys (anything that can be interpolated, except rotations).
    #[binrw::binrw]
    pub struct KeyGroup<T: binrw::BinRead + binrw::BinWrite + 'static>
    where
        T: for<'a> binrw::BinRead<Args<'a> = ()>,
        T: for<'a> binrw::BinWrite<Args<'a> = ()>,
    {
        /// Number of keys in the array.
        pub num_keys: u32,
        /// The key type.
        pub interpolation: KeyType,
        /// The keys.
        #[br(count = num_keys)]
        pub keys: Vec<Key<T>>,
    }
    /*
     * Struct {
     *     name: "QuatKey",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: Some(
     *         "true",
     *     ),
     *     fields: [
     *         StructField {
     *             name: "Time",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "10.1.0.0",
     *             ),
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Time the key applies.",
     *             ),
     *         },
     *         StructField {
     *             name: "Time",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "#ARG# != 4",
     *             ),
     *             until: None,
     *             since: Some(
     *                 "10.1.0.106",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Time the key applies.",
     *             ),
     *         },
     *         StructField {
     *             name: "Value",
     *             type: "#T#",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "#ARG# != 4",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Value of the key.",
     *             ),
     *         },
     *         StructField {
     *             name: "TBC",
     *             type: "TBC",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "#ARG# == 3",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The TBC of the key.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A special version of the key type used for quaternions. Never has tangents. #T# should always be Quaternion.",
     *     ),
     * }
     */
    /// A special version of the key type used for quaternions. Never has tangents. #T# should always be Quaternion.
    #[binrw::binrw]
    pub struct QuatKey<T: binrw::BinRead + binrw::BinWrite + 'static>
    where
        T: for<'a> binrw::BinRead<Args<'a> = ()>,
        T: for<'a> binrw::BinWrite<Args<'a> = ()>,
    {
        // field is out of version range (None, Some("10.1.0.0"))
        // pub time: f32,
        /// Time the key applies.
        pub time: f32,
        /// Value of the key.
        pub value: T,
        /// The TBC of the key.
        pub tbc: TBC,
    }
    /*
     * Struct {
     *     name: "TexCoord",
     *     size: Some(
     *         8,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "u",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "First coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "v",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Second coordinate.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "Texture coordinates (u,v). As in OpenGL; image origin is in the lower left corner.",
     *     ),
     * }
     */
    /// Texture coordinates (u,v). As in OpenGL; image origin is in the lower left corner.
    #[binrw::binrw]
    pub struct TexCoord {
        /// First coordinate.
        pub u: f32,
        /// Second coordinate.
        pub v: f32,
    }
    /*
     * Struct {
     *     name: "HalfTexCoord",
     *     size: Some(
     *         4,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "u",
     *             type: "hfloat",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "First coordinate.",
     *             ),
     *         },
     *         StructField {
     *             name: "v",
     *             type: "hfloat",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Second coordinate.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "Texture coordinates (u,v).",
     *     ),
     * }
     */
    /// Texture coordinates (u,v).
    #[binrw::binrw]
    pub struct HalfTexCoord {
        /// First coordinate.
        pub u: Float16,
        /// Second coordinate.
        pub v: Float16,
    }
    /*
     * Struct {
     *     name: "TexDesc",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Image",
     *             type: "Ref",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "3.1",
     *             ),
     *             since: None,
     *             template: Some(
     *                 "NiImage",
     *             ),
     *             description: Some(
     *                 "Link to the texture image.",
     *             ),
     *         },
     *         StructField {
     *             name: "Source",
     *             type: "Ref",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "3.3.0.13",
     *             ),
     *             template: Some(
     *                 "NiSourceTexture",
     *             ),
     *             description: Some(
     *                 "NiSourceTexture object index.",
     *             ),
     *         },
     *         StructField {
     *             name: "Clamp Mode",
     *             type: "TexClampMode",
     *             length: None,
     *             default: Some(
     *                 "WRAP_S_WRAP_T",
     *             ),
     *             cond: None,
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "0=clamp S clamp T, 1=clamp S wrap T, 2=wrap S clamp T, 3=wrap S wrap T",
     *             ),
     *         },
     *         StructField {
     *             name: "Filter Mode",
     *             type: "TexFilterMode",
     *             length: None,
     *             default: Some(
     *                 "FILTER_TRILERP",
     *             ),
     *             cond: None,
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "0=nearest, 1=bilinear, 2=trilinear, 3=..., 4=..., 5=...",
     *             ),
     *         },
     *         StructField {
     *             name: "Flags",
     *             type: "TexturingMapFlags",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "20.1.0.3",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Texture mode flags; clamp and filter mode stored in upper byte with 0xYZ00 = clamp mode Y, filter mode Z.",
     *             ),
     *         },
     *         StructField {
     *             name: "Max Anisotropy",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "20.5.0.4",
     *             ),
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "UV Set",
     *             type: "uint",
     *             length: None,
     *             default: Some(
     *                 "0",
     *             ),
     *             cond: None,
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The texture coordinate set in NiGeometryData that this texture slot will use.",
     *             ),
     *         },
     *         StructField {
     *             name: "PS2 L",
     *             type: "short",
     *             length: None,
     *             default: Some(
     *                 "0",
     *             ),
     *             cond: None,
     *             until: Some(
     *                 "10.4.0.1",
     *             ),
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "L can range from 0 to 3 and are used to specify how fast a texture gets blurry.",
     *             ),
     *         },
     *         StructField {
     *             name: "PS2 K",
     *             type: "short",
     *             length: None,
     *             default: Some(
     *                 "-75",
     *             ),
     *             cond: None,
     *             until: Some(
     *                 "10.4.0.1",
     *             ),
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "K is used as an offset into the mipmap levels and can range from -2047 to 2047. Positive values push the mipmap towards being blurry and negative values make the mipmap sharper.",
     *             ),
     *         },
     *         StructField {
     *             name: "Unknown Short 1",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "4.1.0.12",
     *             ),
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Has Texture Transform",
     *             type: "bool",
     *             length: None,
     *             default: Some(
     *                 "false",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Whether or not the texture coordinates are transformed.",
     *             ),
     *         },
     *         StructField {
     *             name: "Translation",
     *             type: "TexCoord",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "Has Texture Transform",
     *             ),
     *             until: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "The UV translation.",
     *             ),
     *         },
     *         StructField {
     *             name: "Scale",
     *             type: "TexCoord",
     *             length: None,
     *             default: Some(
     *                 "#VEC2_ONE#",
     *             ),
     *             cond: Some(
     *                 "Has Texture Transform",
     *             ),
     *             until: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "The UV scale.",
     *             ),
     *         },
     *         StructField {
     *             name: "Rotation",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "0.0",
     *             ),
     *             cond: Some(
     *                 "Has Texture Transform",
     *             ),
     *             until: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "The W axis rotation in texture space.",
     *             ),
     *         },
     *         StructField {
     *             name: "Transform Method",
     *             type: "TransformMethod",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "Has Texture Transform",
     *             ),
     *             until: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Depending on the source, scaling can occur before or after rotation.",
     *             ),
     *         },
     *         StructField {
     *             name: "Center",
     *             type: "TexCoord",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "Has Texture Transform",
     *             ),
     *             until: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "The origin around which the texture rotates.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "NiTexturingProperty::Map. Texture description.",
     *     ),
     * }
     */
    /// NiTexturingProperty::Map. Texture description.
    #[binrw::binrw]
    pub struct TexDesc {
        // field is out of version range (None, Some("3.1"))
        // pub image: Ref<NiImage>,
        /// NiSourceTexture object index.
        pub source: Ref<NiSourceTexture>,
        /// 0=clamp S clamp T, 1=clamp S wrap T, 2=wrap S clamp T, 3=wrap S wrap T
        pub clamp_mode: TexClampMode,
        /// 0=nearest, 1=bilinear, 2=trilinear, 3=..., 4=..., 5=...
        pub filter_mode: TexFilterMode,
        // field is out of version range (Some("20.1.0.3"), None)
        // pub flags: TexturingMapFlags,
        // field is out of version range (Some("20.5.0.4"), None)
        // pub max_anisotropy: u16,
        /// The texture coordinate set in NiGeometryData that this texture slot will use.
        pub uv_set: u32,
        // field is out of version range (None, Some("10.4.0.1"))
        // pub ps2_l: short,
        // field is out of version range (None, Some("10.4.0.1"))
        // pub ps2_k: short,
        // field is out of version range (None, Some("4.1.0.12"))
        // pub unknown_short_1: u16,
        /// Whether or not the texture coordinates are transformed.
        pub has_texture_transform: u8,
        /// The UV translation.
        pub translation: TexCoord,
        /// The UV scale.
        pub scale: TexCoord,
        /// The W axis rotation in texture space.
        pub rotation: f32,
        /// Depending on the source, scaling can occur before or after rotation.
        pub transform_method: TransformMethod,
        /// The origin around which the texture rotates.
        pub center: TexCoord,
    }
    /*
     * Struct {
     *     name: "ShaderTexDesc",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Has Map",
     *             type: "bool",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Map",
     *             type: "TexDesc",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "Has Map",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Map ID",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "Has Map",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Unique identifier for the Gamebryo shader system.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "NiTexturingProperty::ShaderMap. Shader texture description.",
     *     ),
     * }
     */
    /// NiTexturingProperty::ShaderMap. Shader texture description.
    #[binrw::binrw]
    pub struct ShaderTexDesc {
        pub has_map: u8,
        pub map: TexDesc,
        /// Unique identifier for the Gamebryo shader system.
        pub map_id: u32,
    }
    /*
     * Struct {
     *     name: "Triangle",
     *     size: Some(
     *         6,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "v1",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "First vertex index.",
     *             ),
     *         },
     *         StructField {
     *             name: "v2",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Second vertex index.",
     *             ),
     *         },
     *         StructField {
     *             name: "v3",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Third vertex index.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "List of three vertex indices.",
     *     ),
     * }
     */
    /// List of three vertex indices.
    #[binrw::binrw]
    pub struct Triangle {
        /// First vertex index.
        pub v1: u16,
        /// Second vertex index.
        pub v2: u16,
        /// Third vertex index.
        pub v3: u16,
    }
    /*
     * Struct {
     *     name: "SkinPartition",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Num Vertices",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Number of vertices in this submesh.",
     *             ),
     *         },
     *         StructField {
     *             name: "Num Triangles",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Number of triangles in this submesh.",
     *             ),
     *         },
     *         StructField {
     *             name: "Num Bones",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Number of bones influencing this submesh.",
     *             ),
     *         },
     *         StructField {
     *             name: "Num Strips",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Number of strips in this submesh (zero if not stripped).",
     *             ),
     *         },
     *         StructField {
     *             name: "Num Weights Per Vertex",
     *             type: "ushort",
     *             length: None,
     *             default: Some(
     *                 "4",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Number of weight coefficients per vertex. The Gamebryo engine seems to work well only if this number is equal to 4, even if there are less than 4 influences per vertex.",
     *             ),
     *         },
     *         StructField {
     *             name: "Bones",
     *             type: "ushort",
     *             length: Some(
     *                 "Num Bones",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "List of bones.",
     *             ),
     *         },
     *         StructField {
     *             name: "Has Vertex Map",
     *             type: "bool",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Do we have a vertex map?",
     *             ),
     *         },
     *         StructField {
     *             name: "Vertex Map",
     *             type: "ushort",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "10.0.1.2",
     *             ),
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Maps the weight/influence lists in this submesh to the vertices in the shape being skinned.",
     *             ),
     *         },
     *         StructField {
     *             name: "Vertex Map",
     *             type: "ushort",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             default: None,
     *             cond: Some(
     *                 "Has Vertex Map",
     *             ),
     *             until: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Maps the weight/influence lists in this submesh to the vertices in the shape being skinned.",
     *             ),
     *         },
     *         StructField {
     *             name: "Has Vertex Weights",
     *             type: "bool",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Do we have vertex weights?",
     *             ),
     *         },
     *         StructField {
     *             name: "Vertex Weights",
     *             type: "float",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "10.0.1.2",
     *             ),
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The vertex weights.",
     *             ),
     *         },
     *         StructField {
     *             name: "Vertex Weights",
     *             type: "float",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             default: None,
     *             cond: Some(
     *                 "Has Vertex Weights",
     *             ),
     *             until: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "The vertex weights.",
     *             ),
     *         },
     *         StructField {
     *             name: "Strip Lengths",
     *             type: "ushort",
     *             length: Some(
     *                 "Num Strips",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The strip lengths.",
     *             ),
     *         },
     *         StructField {
     *             name: "Has Faces",
     *             type: "bool",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Do we have triangle or strip data?",
     *             ),
     *         },
     *         StructField {
     *             name: "Strips",
     *             type: "ushort",
     *             length: Some(
     *                 "Num Strips",
     *             ),
     *             default: None,
     *             cond: Some(
     *                 "Num Strips != 0",
     *             ),
     *             until: Some(
     *                 "10.0.1.2",
     *             ),
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The strips.",
     *             ),
     *         },
     *         StructField {
     *             name: "Strips",
     *             type: "ushort",
     *             length: Some(
     *                 "Num Strips",
     *             ),
     *             default: None,
     *             cond: Some(
     *                 "(Has Faces) #AND# (Num Strips != 0)",
     *             ),
     *             until: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "The strips.",
     *             ),
     *         },
     *         StructField {
     *             name: "Triangles",
     *             type: "Triangle",
     *             length: Some(
     *                 "Num Triangles",
     *             ),
     *             default: None,
     *             cond: Some(
     *                 "Num Strips == 0",
     *             ),
     *             until: Some(
     *                 "10.0.1.2",
     *             ),
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The triangles.",
     *             ),
     *         },
     *         StructField {
     *             name: "Triangles",
     *             type: "Triangle",
     *             length: Some(
     *                 "Num Triangles",
     *             ),
     *             default: None,
     *             cond: Some(
     *                 "(Has Faces) #AND# (Num Strips == 0)",
     *             ),
     *             until: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "The triangles.",
     *             ),
     *         },
     *         StructField {
     *             name: "Has Bone Indices",
     *             type: "bool",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Do we have bone indices?",
     *             ),
     *         },
     *         StructField {
     *             name: "Bone Indices",
     *             type: "byte",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             default: None,
     *             cond: Some(
     *                 "Has Bone Indices",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Bone indices, they index into 'Bones'.",
     *             ),
     *         },
     *         StructField {
     *             name: "LOD Level",
     *             type: "byte",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Global VB",
     *             type: "bool",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Vertex Desc",
     *             type: "BSVertexDesc",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Triangles Copy",
     *             type: "Triangle",
     *             length: Some(
     *                 "Num Triangles",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: Some(
     *         "Skinning data for a submesh, optimized for hardware skinning. Part of NiSkinPartition.",
     *     ),
     * }
     */
    /// Skinning data for a submesh, optimized for hardware skinning. Part of NiSkinPartition.
    #[binrw::binrw]
    pub struct SkinPartition {
        /// Number of vertices in this submesh.
        pub num_vertices: u16,
        /// Number of triangles in this submesh.
        pub num_triangles: u16,
        /// Number of bones influencing this submesh.
        pub num_bones: u16,
        /// Number of strips in this submesh (zero if not stripped).
        pub num_strips: u16,
        /// Number of weight coefficients per vertex. The Gamebryo engine seems to work well only if this number is equal to 4, even if there are less than 4 influences per vertex.
        pub num_weights_per_vertex: u16,
        /// List of bones.
        #[br(count = num_bones)]
        pub bones: Vec<u16>,
        /// Do we have a vertex map?
        pub has_vertex_map: u8,
        // field is out of version range (None, Some("10.0.1.2"))
        // pub vertex_map: Vec<u16>,
        /// Maps the weight/influence lists in this submesh to the vertices in the shape being skinned.
        #[br(count = num_vertices)]
        pub vertex_map: Vec<u16>,
        /// Do we have vertex weights?
        pub has_vertex_weights: u8,
        // field is out of version range (None, Some("10.0.1.2"))
        // pub vertex_weights: Vec<f32>,
        /// The vertex weights.
        #[br(count = num_vertices)]
        pub vertex_weights: Vec<f32>,
        /// The strip lengths.
        #[br(count = num_strips)]
        pub strip_lengths: Vec<u16>,
        /// Do we have triangle or strip data?
        pub has_faces: u8,
        // field is out of version range (None, Some("10.0.1.2"))
        // pub strips: Vec<u16>,
        /// The strips.
        #[br(count = num_strips)]
        pub strips: Vec<u16>,
        // field is out of version range (None, Some("10.0.1.2"))
        // pub triangles: Vec<Triangle>,
        /// The triangles.
        #[br(count = num_triangles)]
        pub triangles: Vec<Triangle>,
        /// Do we have bone indices?
        pub has_bone_indices: u8,
        /// Bone indices, they index into 'Bones'.
        #[br(count = num_vertices)]
        pub bone_indices: Vec<u8>,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub lod_level: u8,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub global_vb: u8,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub vertex_desc: BSVertexDesc,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub triangles_copy: Vec<Triangle>,
    }
    /*
     * Struct {
     *     name: "NiPlane",
     *     size: Some(
     *         16,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Normal",
     *             type: "Vector3",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The plane normal.",
     *             ),
     *         },
     *         StructField {
     *             name: "Constant",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The plane constant.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "A plane.",
     *     ),
     * }
     */
    /// A plane.
    #[binrw::binrw]
    pub struct NiPlane {
        /// The plane normal.
        pub normal: Vector3,
        /// The plane constant.
        pub constant: f32,
    }
    /*
     * Struct {
     *     name: "NiBoundAABB",
     *     size: Some(
     *         26,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Num Corners",
     *             type: "ushort",
     *             length: None,
     *             default: Some(
     *                 "2",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Corners",
     *             type: "Vector3",
     *             length: Some(
     *                 "2",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Corners are only non-zero if Num Corners is 2. Hardcoded to 2.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "Divinity 2 specific NiBound extension.",
     *     ),
     * }
     */
    /// Divinity 2 specific NiBound extension.
    #[binrw::binrw]
    pub struct NiBoundAABB {
        pub num_corners: u16,
        /// Corners are only non-zero if Num Corners is 2. Hardcoded to 2.
        #[br(count = 2)]
        pub corners: Vec<Vector3>,
    }
    /*
     * Struct {
     *     name: "NiBound",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Center",
     *             type: "Vector3",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The sphere's center.",
     *             ),
     *         },
     *         StructField {
     *             name: "Radius",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The sphere's radius.",
     *             ),
     *         },
     *         StructField {
     *             name: "DIV2 AABB",
     *             type: "NiBoundAABB",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "20.3.0.9",
     *             ),
     *             since: Some(
     *                 "20.3.0.9",
     *             ),
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: Some(
     *         "A sphere.",
     *     ),
     * }
     */
    /// A sphere.
    #[binrw::binrw]
    pub struct NiBound {
        /// The sphere's center.
        pub center: Vector3,
        /// The sphere's radius.
        pub radius: f32,
        // field is out of version range (Some("20.3.0.9"), Some("20.3.0.9"))
        // pub div2_aabb: NiBoundAABB,
    }
    /*
     * Struct {
     *     name: "NiCurve3",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Degree",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Num Control Points",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Control Points",
     *             type: "Vector3",
     *             length: Some(
     *                 "Num Control Points",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Num Knots",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Knots",
     *             type: "float",
     *             length: Some(
     *                 "Num Knots",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: Some(
     *         "A 3D curve made up of control points and knots.",
     *     ),
     * }
     */
    /// A 3D curve made up of control points and knots.
    #[binrw::binrw]
    pub struct NiCurve3 {
        pub degree: u32,
        pub num_control_points: u32,
        #[br(count = num_control_points)]
        pub control_points: Vec<Vector3>,
        pub num_knots: u32,
        #[br(count = num_knots)]
        pub knots: Vec<f32>,
    }
    /*
     * Struct {
     *     name: "NiQuatTransform",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Translation",
     *             type: "Vector3",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Rotation",
     *             type: "Quaternion",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Scale",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "TRS Valid",
     *             type: "bool",
     *             length: Some(
     *                 "3",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "10.1.0.109",
     *             ),
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Whether each transform component is valid.",
     *             ),
     *         },
     *     ],
     *     description: None,
     * }
     */
    #[binrw::binrw]
    pub struct NiQuatTransform {
        pub translation: Vector3,
        pub rotation: Quaternion,
        pub scale: f32,
        // field is out of version range (None, Some("10.1.0.109"))
        // pub trs_valid: Vec<u8>,
    }
    /*
     * Struct {
     *     name: "NiTransform",
     *     size: Some(
     *         52,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Rotation",
     *             type: "Matrix33",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The rotation part of the transformation matrix.",
     *             ),
     *         },
     *         StructField {
     *             name: "Translation",
     *             type: "Vector3",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The translation vector.",
     *             ),
     *         },
     *         StructField {
     *             name: "Scale",
     *             type: "float",
     *             length: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Scaling part (only uniform scaling is supported).",
     *             ),
     *         },
     *     ],
     *     description: None,
     * }
     */
    #[binrw::binrw]
    pub struct NiTransform {
        /// The rotation part of the transformation matrix.
        pub rotation: Matrix33,
        /// The translation vector.
        pub translation: Vector3,
        /// Scaling part (only uniform scaling is supported).
        pub scale: f32,
    }
    /*
     * Struct {
     *     name: "NiParticleInfo",
     *     size: Some(
     *         40,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Velocity",
     *             type: "Vector3",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Particle direction and speed.",
     *             ),
     *         },
     *         StructField {
     *             name: "Rotation Axis",
     *             type: "Vector3",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "10.4.0.1",
     *             ),
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Age",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Life Span",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Last Update",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Timestamp of the last update.",
     *             ),
     *         },
     *         StructField {
     *             name: "Spawn Generation",
     *             type: "ushort",
     *             length: None,
     *             default: Some(
     *                 "0",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Code",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Usually matches array index",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "Called NiPerParticleData in NiOldParticles.\r\n        Holds the state of a particle at the time the system was saved.",
     *     ),
     * }
     */
    /// Called NiPerParticleData in NiOldParticles.
    /// Holds the state of a particle at the time the system was saved.
    #[binrw::binrw]
    pub struct NiParticleInfo {
        /// Particle direction and speed.
        pub velocity: Vector3,
        // field is out of version range (None, Some("10.4.0.1"))
        // pub rotation_axis: Vector3,
        pub age: f32,
        pub life_span: f32,
        /// Timestamp of the last update.
        pub last_update: f32,
        pub spawn_generation: u16,
        /// Usually matches array index
        pub code: u16,
    }
    /*
     * Struct {
     *     name: "BoneData",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Skin Transform",
     *             type: "NiTransform",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Offset of the skin from this bone in bind position.",
     *             ),
     *         },
     *         StructField {
     *             name: "Bounding Sphere",
     *             type: "NiBound",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Note that its a Sphere Containing Axis Aligned Box not a minimum volume Sphere",
     *             ),
     *         },
     *         StructField {
     *             name: "Num Vertices",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Number of weighted vertices.",
     *             ),
     *         },
     *         StructField {
     *             name: "Vertex Weights",
     *             type: "BoneVertData",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "4.2.1.0",
     *             ),
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The vertex weights.",
     *             ),
     *         },
     *         StructField {
     *             name: "Vertex Weights",
     *             type: "BoneVertData",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             default: None,
     *             cond: Some(
     *                 "#ARG# != 0",
     *             ),
     *             until: None,
     *             since: Some(
     *                 "4.2.2.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "The vertex weights.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "NiSkinData::BoneData. Skinning data component.",
     *     ),
     * }
     */
    /// NiSkinData::BoneData. Skinning data component.
    #[binrw::binrw]
    pub struct BoneData {
        /// Offset of the skin from this bone in bind position.
        pub skin_transform: NiTransform,
        /// Note that its a Sphere Containing Axis Aligned Box not a minimum volume Sphere
        pub bounding_sphere: NiBound,
        /// Number of weighted vertices.
        pub num_vertices: u16,
        // field is out of version range (None, Some("4.2.1.0"))
        // pub vertex_weights: Vec<BoneVertData>,
        /// The vertex weights.
        #[br(count = num_vertices)]
        pub vertex_weights: Vec<BoneVertData>,
    }
    /*
     * Struct {
     *     name: "OldSkinData",
     *     size: Some(
     *         18,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Vertex Weight",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The amount that this bone affects the vertex.",
     *             ),
     *         },
     *         StructField {
     *             name: "Vertex Index",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "The index of the vertex that this weight applies to.",
     *             ),
     *         },
     *         StructField {
     *             name: "Unknown Vector",
     *             type: "Vector3",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: Some(
     *         "Used to store skin weights in NiTriShapeSkinController.",
     *     ),
     * }
     */
    /// Used to store skin weights in NiTriShapeSkinController.
    #[binrw::binrw]
    pub struct OldSkinData {
        /// The amount that this bone affects the vertex.
        pub vertex_weight: f32,
        /// The index of the vertex that this weight applies to.
        pub vertex_index: u16,
        pub unknown_vector: Vector3,
    }
    /*
     * Struct {
     *     name: "BoxBV",
     *     size: Some(
     *         60,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Center",
     *             type: "Vector3",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Axis",
     *             type: "Vector3",
     *             length: Some(
     *                 "3",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Extent",
     *             type: "Vector3",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: Some(
     *         "Box Bounding Volume",
     *     ),
     * }
     */
    /// Box Bounding Volume
    #[binrw::binrw]
    pub struct BoxBV {
        pub center: Vector3,
        #[br(count = 3)]
        pub axis: Vec<Vector3>,
        pub extent: Vector3,
    }
    /*
     * Struct {
     *     name: "CapsuleBV",
     *     size: Some(
     *         32,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Center",
     *             type: "Vector3",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Origin",
     *             type: "Vector3",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Extent",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Radius",
     *             type: "float",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: Some(
     *         "Capsule Bounding Volume",
     *     ),
     * }
     */
    /// Capsule Bounding Volume
    #[binrw::binrw]
    pub struct CapsuleBV {
        pub center: Vector3,
        pub origin: Vector3,
        pub extent: f32,
        pub radius: f32,
    }
    /*
     * Struct {
     *     name: "HalfSpaceBV",
     *     size: Some(
     *         28,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Plane",
     *             type: "NiPlane",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Center",
     *             type: "Vector3",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: None,
     * }
     */
    #[binrw::binrw]
    pub struct HalfSpaceBV {
        pub plane: NiPlane,
        pub center: Vector3,
    }
    /*
     * Struct {
     *     name: "BoundingVolume",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Collision Type",
     *             type: "BoundVolumeType",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Type of collision data.",
     *             ),
     *         },
     *         StructField {
     *             name: "Sphere",
     *             type: "NiBound",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "Collision Type == 0",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Box",
     *             type: "BoxBV",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "Collision Type == 1",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Capsule",
     *             type: "CapsuleBV",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "Collision Type == 2",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Union BV",
     *             type: "UnionBV",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "Collision Type == 4",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Half Space",
     *             type: "HalfSpaceBV",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "Collision Type == 5",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: None,
     * }
     */
    #[binrw::binrw]
    pub struct BoundingVolume {
        /// Type of collision data.
        pub collision_type: BoundVolumeType,
        pub sphere: NiBound,
        pub r#box: BoxBV,
        pub capsule: CapsuleBV,
        pub union_bv: UnionBV,
        pub half_space: HalfSpaceBV,
    }
    /*
     * Struct {
     *     name: "UnionBV",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Num BV",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Bounding Volumes",
     *             type: "BoundingVolume",
     *             length: Some(
     *                 "Num BV",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: None,
     * }
     */
    #[binrw::binrw]
    pub struct UnionBV {
        pub num_bv: u32,
        #[br(count = num_bv)]
        pub bounding_volumes: Vec<BoundingVolume>,
    }
    /*
     * Struct {
     *     name: "MaterialData",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Has Shader",
     *             type: "bool",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "20.1.0.3",
     *             ),
     *             since: Some(
     *                 "10.0.1.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Shader.",
     *             ),
     *         },
     *         StructField {
     *             name: "Shader Name",
     *             type: "string",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "Has Shader",
     *             ),
     *             until: Some(
     *                 "20.1.0.3",
     *             ),
     *             since: Some(
     *                 "10.0.1.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "The shader name.",
     *             ),
     *         },
     *         StructField {
     *             name: "Shader Extra Data",
     *             type: "int",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "Has Shader",
     *             ),
     *             until: Some(
     *                 "20.1.0.3",
     *             ),
     *             since: Some(
     *                 "10.0.1.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Extra data associated with the shader. A value of -1 means the shader is the default implementation.",
     *             ),
     *         },
     *         StructField {
     *             name: "Num Materials",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "20.2.0.5",
     *             ),
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Material Name",
     *             type: "NiFixedString",
     *             length: Some(
     *                 "Num Materials",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "20.2.0.5",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "The name of the material.",
     *             ),
     *         },
     *         StructField {
     *             name: "Material Extra Data",
     *             type: "int",
     *             length: Some(
     *                 "Num Materials",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "20.2.0.5",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Extra data associated with the material. A value of -1 means the material is the default implementation.",
     *             ),
     *         },
     *         StructField {
     *             name: "Active Material",
     *             type: "int",
     *             length: None,
     *             default: Some(
     *                 "-1",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "20.2.0.5",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "The index of the currently active material.",
     *             ),
     *         },
     *         StructField {
     *             name: "Cyanide Unknown",
     *             type: "byte",
     *             length: None,
     *             default: Some(
     *                 "255",
     *             ),
     *             cond: None,
     *             until: Some(
     *                 "10.2.0.0",
     *             ),
     *             since: Some(
     *                 "10.2.0.0",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Cyanide extension (Blood Bowl).",
     *             ),
     *         },
     *         StructField {
     *             name: "WorldShift Unknown",
     *             type: "int",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: Some(
     *                 "10.4.0.1",
     *             ),
     *             since: Some(
     *                 "10.3.0.1",
     *             ),
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Material Needs Update",
     *             type: "bool",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             description: Some(
     *                 "Whether the materials for this object always needs to be updated before rendering with them.",
     *             ),
     *         },
     *     ],
     *     description: None,
     * }
     */
    #[binrw::binrw]
    pub struct MaterialData {
        /// Shader.
        pub has_shader: u8,
        /// The shader name.
        pub shader_name: string,
        /// Extra data associated with the shader. A value of -1 means the shader is the default implementation.
        pub shader_extra_data: i32,
        // field is out of version range (Some("20.2.0.5"), None)
        // pub num_materials: u32,
        // field is out of version range (Some("20.2.0.5"), None)
        // pub material_name: Vec<NiFixedString>,
        // field is out of version range (Some("20.2.0.5"), None)
        // pub material_extra_data: Vec<i32>,
        // field is out of version range (Some("20.2.0.5"), None)
        // pub active_material: i32,
        // field is out of version range (Some("10.2.0.0"), Some("10.2.0.0"))
        // pub cyanide_unknown: u8,
        // field is out of version range (Some("10.3.0.1"), Some("10.4.0.1"))
        // pub worldshift_unknown: i32,
        // field is out of version range (Some("20.2.0.7"), None)
        // pub material_needs_update: u8,
    }
    /*
     * Struct {
     *     name: "PixelFormatComponent",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Type",
     *             type: "PixelComponent",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Component Type",
     *             ),
     *         },
     *         StructField {
     *             name: "Convention",
     *             type: "PixelRepresentation",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Data Storage Convention",
     *             ),
     *         },
     *         StructField {
     *             name: "Bits Per Channel",
     *             type: "byte",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Bits per component",
     *             ),
     *         },
     *         StructField {
     *             name: "Is Signed",
     *             type: "bool",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: None,
     * }
     */
    #[binrw::binrw]
    pub struct PixelFormatComponent {
        /// Component Type
        pub r#type: PixelComponent,
        /// Data Storage Convention
        pub convention: PixelRepresentation,
        /// Bits per component
        pub bits_per_channel: u8,
        pub is_signed: u8,
    }
    /*
     * Struct {
     *     name: "FormatPrefs",
     *     size: Some(
     *         12,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Pixel Layout",
     *             type: "PixelLayout",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Requests the way the image will be stored.",
     *             ),
     *         },
     *         StructField {
     *             name: "Use Mipmaps",
     *             type: "MipMapFormat",
     *             length: None,
     *             default: Some(
     *                 "MIP_FMT_DEFAULT",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Requests if mipmaps are used or not.",
     *             ),
     *         },
     *         StructField {
     *             name: "Alpha Format",
     *             type: "AlphaFormat",
     *             length: None,
     *             default: Some(
     *                 "ALPHA_DEFAULT",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Requests no alpha, 1-bit alpha, or",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "NiTexture::FormatPrefs. These preferences are a request to the renderer to use a format the most closely matches the settings and may be ignored.",
     *     ),
     * }
     */
    /// NiTexture::FormatPrefs. These preferences are a request to the renderer to use a format the most closely matches the settings and may be ignored.
    #[binrw::binrw]
    pub struct FormatPrefs {
        /// Requests the way the image will be stored.
        pub pixel_layout: PixelLayout,
        /// Requests if mipmaps are used or not.
        pub use_mipmaps: MipMapFormat,
        /// Requests no alpha, 1-bit alpha, or
        pub alpha_format: AlphaFormat,
    }
    /*
     * Struct {
     *     name: "Polygon",
     *     size: Some(
     *         8,
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Num Vertices",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Vertex Offset",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Offset in vertex array.",
     *             ),
     *         },
     *         StructField {
     *             name: "Num Triangles",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Triangle Offset",
     *             type: "ushort",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Offset in indices array.",
     *             ),
     *         },
     *     ],
     *     description: Some(
     *         "Two dimensional screen elements.",
     *     ),
     * }
     */
    /// Two dimensional screen elements.
    #[binrw::binrw]
    pub struct Polygon {
        pub num_vertices: u16,
        /// Offset in vertex array.
        pub vertex_offset: u16,
        pub num_triangles: u16,
        /// Offset in indices array.
        pub triangle_offset: u16,
    }
    /*
     * Struct {
     *     name: "NiAGDDataStream",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Type",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Type of data in this channel",
     *             ),
     *         },
     *         StructField {
     *             name: "Unit Size",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Number of bytes per element of this channel",
     *             ),
     *         },
     *         StructField {
     *             name: "Total Size",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Total number of bytes of this channel (num vertices times num bytes per element)",
     *             ),
     *         },
     *         StructField {
     *             name: "Stride",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Number of bytes per element in all channels together. Sum of num channel bytes per element over all block infos.",
     *             ),
     *         },
     *         StructField {
     *             name: "Block Index",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Unsure. The block in which this channel is stored? Usually there is only one block, and so this is zero.",
     *             ),
     *         },
     *         StructField {
     *             name: "Block Offset",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: Some(
     *                 "Offset (in bytes) of this channel. Sum of all num channel bytes per element of all preceeding block infos.",
     *             ),
     *         },
     *         StructField {
     *             name: "Flags",
     *             type: "NiAGDDataStreamFlags",
     *             length: None,
     *             default: Some(
     *                 "2",
     *             ),
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: None,
     * }
     */
    #[binrw::binrw]
    pub struct NiAGDDataStream {
        /// Type of data in this channel
        pub r#type: u32,
        /// Number of bytes per element of this channel
        pub unit_size: u32,
        /// Total number of bytes of this channel (num vertices times num bytes per element)
        pub total_size: u32,
        /// Number of bytes per element in all channels together. Sum of num channel bytes per element over all block infos.
        pub stride: u32,
        /// Unsure. The block in which this channel is stored? Usually there is only one block, and so this is zero.
        pub block_index: u32,
        /// Offset (in bytes) of this channel. Sum of all num channel bytes per element of all preceeding block infos.
        pub block_offset: u32,
        pub flags: NiAGDDataStreamFlags,
    }
    /*
     * Struct {
     *     name: "NiAGDDataBlock",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Block Size",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Num Blocks",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Block Offsets",
     *             type: "uint",
     *             length: Some(
     *                 "Num Blocks",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Num Data",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Data Sizes",
     *             type: "uint",
     *             length: Some(
     *                 "Num Data",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Data",
     *             type: "byte",
     *             length: Some(
     *                 "Num Data",
     *             ),
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Shader Index",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "#ARG# == 1",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Total Size",
     *             type: "uint",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "#ARG# == 1",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: None,
     * }
     */
    #[binrw::binrw]
    pub struct NiAGDDataBlock {
        pub block_size: u32,
        pub num_blocks: u32,
        #[br(count = num_blocks)]
        pub block_offsets: Vec<u32>,
        pub num_data: u32,
        #[br(count = num_data)]
        pub data_sizes: Vec<u32>,
        #[br(count = num_data)]
        pub data: Vec<u8>,
        pub shader_index: u32,
        pub total_size: u32,
    }
    /*
     * Struct {
     *     name: "NiAGDDataBlocks",
     *     size: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     generic: None,
     *     fields: [
     *         StructField {
     *             name: "Has Data",
     *             type: "bool",
     *             length: None,
     *             default: None,
     *             cond: None,
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *         StructField {
     *             name: "Data Block",
     *             type: "NiAGDDataBlock",
     *             length: None,
     *             default: None,
     *             cond: Some(
     *                 "Has Data",
     *             ),
     *             until: None,
     *             since: None,
     *             template: None,
     *             description: None,
     *         },
     *     ],
     *     description: None,
     * }
     */
    #[binrw::binrw]
    pub struct NiAGDDataBlocks {
        pub has_data: u8,
        pub data_block: NiAGDDataBlock,
    }
    /*
     * NiObject {
     *     name: "NiObject",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: None,
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Abstract object type.",
     *     ),
     *     fields: [],
     * }
     */
    /// Abstract object type.
    #[binrw::binrw]
    pub struct NiObject {}
    /*
     * NiObject {
     *     name: "NiExtraData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "A generic extra data object.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Name",
     *             type: "string",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.0.1.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Name of this object.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Next Extra Data",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "4.2.2.0",
     *             ),
     *             template: Some(
     *                 "NiExtraData",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Block number of the next extra data object.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Extra Data",
     *             type: "ByteArray",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "3.3.0.13",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The extra data was sometimes stored as binary directly on NiExtraData.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Bytes",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "4.0.0.0",
     *             ),
     *             until: Some(
     *                 "4.2.2.0",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Ignore binary data after 4.x as the child block will cover it.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// A generic extra data object.
    #[binrw::binrw]
    pub struct NiExtraData {
        pub base: NiObject,
        /// Name of this object.
        pub name: string,
        // field is out of version range (None, Some("4.2.2.0"))
        // pub next_extra_data: Ref<NiExtraData>,
        // field is out of version range (None, Some("3.3.0.13"))
        // pub extra_data: ByteArray,
        // field is out of version range (Some("4.0.0.0"), Some("4.2.2.0"))
        // pub num_bytes: u32,
    }
    /*
     * NiObject {
     *     name: "NiObjectNET",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Abstract base class for NiObjects that support names, extra data, and time controllers.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Shader Type",
     *             type: "BSLightingShaderType",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Configures the main shader path",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Name",
     *             type: "string",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Name of this controllable object, used to refer to the object in .kf files.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Legacy Extra Data",
     *             type: "LegacyExtraData",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "2.3",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Extra Data",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "3.0",
     *             ),
     *             until: Some(
     *                 "4.2.2.0",
     *             ),
     *             template: Some(
     *                 "NiExtraData",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Extra data object index. (The first in a chain)",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Extra Data List",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.0.1.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The number of Extra Data objects referenced through the list.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Extra Data List",
     *             type: "Ref",
     *             length: Some(
     *                 "Num Extra Data List",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.0.1.0",
     *             ),
     *             until: None,
     *             template: Some(
     *                 "NiExtraData",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "List of extra data indices.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Controller",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "3.0",
     *             ),
     *             until: None,
     *             template: Some(
     *                 "NiTimeController",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Controller object index. (The first in a chain)",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Abstract base class for NiObjects that support names, extra data, and time controllers.
    #[binrw::binrw]
    pub struct NiObjectNET {
        pub base: NiObject,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub shader_type: BSLightingShaderType,
        /// Name of this controllable object, used to refer to the object in .kf files.
        pub name: string,
        // field is out of version range (None, Some("2.3"))
        // pub legacy_extra_data: LegacyExtraData,
        // field is out of version range (Some("3.0"), Some("4.2.2.0"))
        // pub extra_data: Ref<NiExtraData>,
        /// The number of Extra Data objects referenced through the list.
        pub num_extra_data_list: u32,
        /// List of extra data indices.
        pub extra_data_list: Ref<NiExtraData>,
        /// Controller object index. (The first in a chain)
        pub controller: Ref<NiTimeController>,
    }
    /*
     * NiObject {
     *     name: "NiCollisionObject",
     *     abstract: None,
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "This is the most common collision object found in NIF files. It acts as a real object that\r\n        is visible and possibly (if the body allows for it) interactive. The node itself\r\n        is simple, it only has three properties.\r\n        For this type of collision object, bhkRigidBody or bhkRigidBodyT is generally used.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Target",
     *             type: "Ptr",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiAVObject",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Index of the AV object referring to this collision object.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// This is the most common collision object found in NIF files. It acts as a real object that
    /// is visible and possibly (if the body allows for it) interactive. The node itself
    /// is simple, it only has three properties.
    /// For this type of collision object, bhkRigidBody or bhkRigidBodyT is generally used.
    #[binrw::binrw]
    pub struct NiCollisionObject {
        pub base: NiObject,
        /// Index of the AV object referring to this collision object.
        pub target: Ptr<NiAVObject>,
    }
    /*
     * NiObject {
     *     name: "NiCollisionData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiCollisionObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Collision box.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Propagation Mode",
     *             type: "PropagationMode",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "PROPAGATE_ALWAYS",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Collision Mode",
     *             type: "CollisionMode",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "NOTEST",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Use ABV",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Use Alternate Bounding Volume.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Bounding Volume",
     *             type: "BoundingVolume",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Use ABV == 1",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Collision box.
    #[binrw::binrw]
    pub struct NiCollisionData {
        pub base: NiCollisionObject,
        pub propagation_mode: PropagationMode,
        pub collision_mode: CollisionMode,
        /// Use Alternate Bounding Volume.
        pub use_abv: u8,
        pub bounding_volume: BoundingVolume,
    }
    /*
     * NiObject {
     *     name: "NiAVObject",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiObjectNET",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Abstract audio-visual base class from which all of Gamebryo's scene graph objects inherit.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Flags",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "0x8000E",
     *             ),
     *             description: Some(
     *                 "Basic flags for AV objects. For Bethesda streams above 26 only.\r\n            ALL: FO4 lacks the 0x80000 flag always. Skyrim lacks it sometimes.\r\n            BSTreeNode: 0x8080E (pre-FO4), 0x400E (FO4)\r\n            BSLeafAnimNode: 0x808000E (pre-FO4), 0x500E (FO4)\r\n            BSDamageStage, BSBlastNode: 0x8000F (pre-FO4), 0x2000000F (FO4)",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Flags",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "3.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Basic flags for AV objects.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Translation",
     *             type: "Vector3",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The translation vector.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Rotation",
     *             type: "Matrix33",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The rotation part of the transformation matrix.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Scale",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             description: Some(
     *                 "Scaling part (only uniform scaling is supported).",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Velocity",
     *             type: "Vector3",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "4.2.2.0",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Unknown function. Always seems to be (0, 0, 0)",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Properties",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Properties",
     *             type: "Ref",
     *             length: Some(
     *                 "Num Properties",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiProperty",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "All rendering properties attached to this object.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Unknown 1",
     *             type: "uint",
     *             length: Some(
     *                 "4",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "2.3",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Unknown 2",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "2.3",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Bounding Volume",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "3.0",
     *             ),
     *             until: Some(
     *                 "4.2.2.0",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Bounding Volume",
     *             type: "BoundingVolume",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Bounding Volume",
     *             ),
     *             since: Some(
     *                 "3.0",
     *             ),
     *             until: Some(
     *                 "4.2.2.0",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Collision Object",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.0.1.0",
     *             ),
     *             until: None,
     *             template: Some(
     *                 "NiCollisionObject",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Abstract audio-visual base class from which all of Gamebryo's scene graph objects inherit.
    #[binrw::binrw]
    pub struct NiAVObject {
        pub base: NiObjectNET,
        /// Basic flags for AV objects. For Bethesda streams above 26 only.
        /// ALL: FO4 lacks the 0x80000 flag always. Skyrim lacks it sometimes.
        /// BSTreeNode: 0x8080E (pre-FO4), 0x400E (FO4)
        /// BSLeafAnimNode: 0x808000E (pre-FO4), 0x500E (FO4)
        /// BSDamageStage, BSBlastNode: 0x8000F (pre-FO4), 0x2000000F (FO4)
        pub flags: u32,
        /// Basic flags for AV objects.
        pub flags: u16,
        /// The translation vector.
        pub translation: Vector3,
        /// The rotation part of the transformation matrix.
        pub rotation: Matrix33,
        /// Scaling part (only uniform scaling is supported).
        pub scale: f32,
        // field is out of version range (None, Some("4.2.2.0"))
        // pub velocity: Vector3,
        pub num_properties: u32,
        /// All rendering properties attached to this object.
        pub properties: Ref<NiProperty>,
        // field is out of version range (None, Some("2.3"))
        // pub unknown_1: u32,
        // field is out of version range (None, Some("2.3"))
        // pub unknown_2: u8,
        // field is out of version range (Some("3.0"), Some("4.2.2.0"))
        // pub has_bounding_volume: u8,
        // field is out of version range (Some("3.0"), Some("4.2.2.0"))
        // pub bounding_volume: BoundingVolume,
        pub collision_object: Ref<NiCollisionObject>,
    }
    /*
     * NiObject {
     *     name: "NiDynamicEffect",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiAVObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Abstract base class for dynamic effects such as NiLights or projected texture effects.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Switch State",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.106",
     *             ),
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "true",
     *             ),
     *             description: Some(
     *                 "If true, then the dynamic effect is applied to affected nodes during rendering.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Affected Nodes",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "4.0.0.2",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Affected Nodes",
     *             type: "Ptr",
     *             length: Some(
     *                 "Num Affected Nodes",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "3.3.0.13",
     *             ),
     *             template: Some(
     *                 "NiNode",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "If a node appears in this list, then its entire subtree will be affected by the effect.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Affected Node Pointers",
     *             type: "uint",
     *             length: Some(
     *                 "Num Affected Nodes",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "4.0.0.0",
     *             ),
     *             until: Some(
     *                 "4.0.0.2",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "As of 4.0 the pointer hash is no longer stored alongside each NiObject on disk, yet this node list still refers to the pointer hashes. Cannot leave the type as Ptr because the link will be invalid.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Affected Nodes",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Affected Nodes",
     *             type: "Ptr",
     *             length: Some(
     *                 "Num Affected Nodes",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: Some(
     *                 "NiNode",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "If a node appears in this list, then its entire subtree will be affected by the effect.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Abstract base class for dynamic effects such as NiLights or projected texture effects.
    #[binrw::binrw]
    pub struct NiDynamicEffect {
        pub base: NiAVObject,
        /// If true, then the dynamic effect is applied to affected nodes during rendering.
        pub switch_state: u8,
        // field is out of version range (None, Some("4.0.0.2"))
        // pub num_affected_nodes: u32,
        // field is out of version range (None, Some("3.3.0.13"))
        // pub affected_nodes: Ptr<NiNode>,
        // field is out of version range (Some("4.0.0.0"), Some("4.0.0.2"))
        // pub affected_node_pointers: u32,
        pub num_affected_nodes: u32,
        /// If a node appears in this list, then its entire subtree will be affected by the effect.
        pub affected_nodes: Ptr<NiNode>,
    }
    /*
     * NiObject {
     *     name: "NiLight",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiDynamicEffect",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Abstract base class that represents light sources in a scene graph.\r\n        For Bethesda Stream 130 (FO4), NiLight now directly inherits from NiAVObject.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Dimmer",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             description: Some(
     *                 "Scales the overall brightness of all light components.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Ambient Color",
     *             type: "Color3",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "#VEC3_ZERO#",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Diffuse Color",
     *             type: "Color3",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "#VEC3_ZERO#",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Specular Color",
     *             type: "Color3",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "#VEC3_ZERO#",
     *             ),
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Abstract base class that represents light sources in a scene graph.
    /// For Bethesda Stream 130 (FO4), NiLight now directly inherits from NiAVObject.
    #[binrw::binrw]
    pub struct NiLight {
        pub base: NiDynamicEffect,
        /// Scales the overall brightness of all light components.
        pub dimmer: f32,
        pub ambient_color: Color3,
        pub diffuse_color: Color3,
        pub specular_color: Color3,
    }
    /*
     * NiObject {
     *     name: "NiProperty",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiObjectNET",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Abstract base class representing all rendering properties. Subclasses are attached to NiAVObjects to control their rendering.",
     *     ),
     *     fields: [],
     * }
     */
    /// Abstract base class representing all rendering properties. Subclasses are attached to NiAVObjects to control their rendering.
    #[binrw::binrw]
    pub struct NiProperty {
        pub base: NiObjectNET,
    }
    /*
     * NiObject {
     *     name: "NiTimeController",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Abstract base class that provides the base timing and update functionality for all the Gamebryo animation controllers.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Next Controller",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiTimeController",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Index of the next controller.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Flags",
     *             type: "TimeControllerFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "76",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Frequency",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             description: Some(
     *                 "Frequency (is usually 1.0).",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Phase",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Phase (usually 0.0).",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Start Time",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "#FLT_MAX#",
     *             ),
     *             description: Some(
     *                 "Controller start time.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Stop Time",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "#FLT_MIN#",
     *             ),
     *             description: Some(
     *                 "Controller stop time.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Target",
     *             type: "Ptr",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "3.3.0.13",
     *             ),
     *             until: None,
     *             template: Some(
     *                 "NiObjectNET",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Controller target (object index of the first controllable ancestor of this object).",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Unknown Integer",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "3.1",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Abstract base class that provides the base timing and update functionality for all the Gamebryo animation controllers.
    #[binrw::binrw]
    pub struct NiTimeController {
        pub base: NiObject,
        /// Index of the next controller.
        pub next_controller: Ref<NiTimeController>,
        pub flags: TimeControllerFlags,
        /// Frequency (is usually 1.0).
        pub frequency: f32,
        /// Phase (usually 0.0).
        pub phase: f32,
        /// Controller start time.
        pub start_time: f32,
        /// Controller stop time.
        pub stop_time: f32,
        /// Controller target (object index of the first controllable ancestor of this object).
        pub target: Ptr<NiObjectNET>,
        // field is out of version range (None, Some("3.1"))
        // pub unknown_integer: u32,
    }
    /*
     * NiObject {
     *     name: "NiGeometry",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiAVObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Describes a visible scene element with vertices like a mesh, a particle system, lines, etc.\r\n            Bethesda 20.2.0.7 NIFs: NiGeometry was changed to BSGeometry. \r\n            Most new blocks (e.g. BSTriShape) do not refer to NiGeometry except NiParticleSystem was changed to use BSGeometry.\r\n            This causes massive inheritance problems so the rows below are doubled up to exclude NiParticleSystem for Bethesda Stream 100+\r\n            and to add data exclusive to BSGeometry.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Bounding Sphere",
     *             type: "NiBound",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Bound Min Max",
     *             type: "float",
     *             length: Some(
     *                 "6",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Skin",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: Some(
     *                 "NiObject",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Data",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiGeometryData",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Data index (NiTriShapeData/NiTriStripData).",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Data",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: Some(
     *                 "NiGeometryData",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Data index (NiTriShapeData/NiTriStripData).",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Skin Instance",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "3.3.0.13",
     *             ),
     *             until: None,
     *             template: Some(
     *                 "NiSkinInstance",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Skin Instance",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: Some(
     *                 "NiSkinInstance",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Material Data",
     *             type: "MaterialData",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.0.1.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Material Data",
     *             type: "MaterialData",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Shader Property",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: Some(
     *                 "BSShaderProperty",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Alpha Property",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: Some(
     *                 "NiAlphaProperty",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Describes a visible scene element with vertices like a mesh, a particle system, lines, etc.
    /// Bethesda 20.2.0.7 NIFs: NiGeometry was changed to BSGeometry.
    /// Most new blocks (e.g. BSTriShape) do not refer to NiGeometry except NiParticleSystem was changed to use BSGeometry.
    /// This causes massive inheritance problems so the rows below are doubled up to exclude NiParticleSystem for Bethesda Stream 100+
    /// and to add data exclusive to BSGeometry.
    #[binrw::binrw]
    pub struct NiGeometry {
        pub base: NiAVObject,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub bounding_sphere: NiBound,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub bound_min_max: f32,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub skin: Ref<NiObject>,
        /// Data index (NiTriShapeData/NiTriStripData).
        pub data: Ref<NiGeometryData>,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub data: Ref<NiGeometryData>,
        pub skin_instance: Ref<NiSkinInstance>,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub skin_instance: Ref<NiSkinInstance>,
        pub material_data: MaterialData,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub material_data: MaterialData,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub shader_property: Ref<BSShaderProperty>,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub alpha_property: Ref<NiAlphaProperty>,
    }
    /*
     * NiObject {
     *     name: "NiTriBasedGeom",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiGeometry",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Describes a mesh, built from triangles.",
     *     ),
     *     fields: [],
     * }
     */
    /// Describes a mesh, built from triangles.
    #[binrw::binrw]
    pub struct NiTriBasedGeom {
        pub base: NiGeometry,
    }
    /*
     * NiObject {
     *     name: "NiGeometryData",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Mesh data: vertices, vertex normals, etc.\r\n            Bethesda 20.2.0.7 NIFs: NiParticlesData no longer inherits from NiGeometryData and inherits NiObject directly. \r\n            \"Num Vertices\" is renamed to \"BS Max Vertices\" for Bethesda 20.2 because Vertices, Normals, Tangents, Colors, and UV arrays\r\n            do not have length for NiPSysData regardless of \"Num\" or booleans.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Group ID",
     *             type: "int",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.114",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Always zero.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Vertices",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Number of vertices.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Vertices",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Number of vertices.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "BS Max Vertices",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Bethesda uses this for max number of particles in NiPSysData.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Keep Flags",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Used with NiCollision objects when OBB or TRI is set.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Compress Flags",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Vertices",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "true",
     *             ),
     *             description: Some(
     *                 "Is the vertex array present? (Always non-zero.)",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Vertices",
     *             type: "Vector3",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             width: None,
     *             cond: Some(
     *                 "Has Vertices",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The mesh vertices.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Data Flags",
     *             type: "NiGeometryDataFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.0.1.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "BS Data Flags",
     *             type: "BSGeometryDataFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Material CRC",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Normals",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Do we have lighting normals? These are essential for proper lighting: if not present, the model will only be influenced by ambient light.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Normals",
     *             type: "Vector3",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             width: None,
     *             cond: Some(
     *                 "Has Normals",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The lighting normals.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Tangents",
     *             type: "Vector3",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             width: None,
     *             cond: Some(
     *                 "(Has Normals) #AND# (((Data Flags #BITOR# BS Data Flags) #BITAND# 4096) != 0)",
     *             ),
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Tangent vectors.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Bitangents",
     *             type: "Vector3",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             width: None,
     *             cond: Some(
     *                 "(Has Normals) #AND# (((Data Flags #BITOR# BS Data Flags) #BITAND# 4096) != 0)",
     *             ),
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Bitangent vectors.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Has DIV2 Floats",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.3.0.9",
     *             ),
     *             until: Some(
     *                 "20.3.0.9",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "DIV2 Floats",
     *             type: "float",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             width: None,
     *             cond: Some(
     *                 "Has DIV2 Floats",
     *             ),
     *             since: Some(
     *                 "20.3.0.9",
     *             ),
     *             until: Some(
     *                 "20.3.0.9",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Bounding Sphere",
     *             type: "NiBound",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Vertex Colors",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Do we have vertex colors? These are usually used to fine-tune the lighting of the model.\r\n\r\n            Note: how vertex colors influence the model can be controlled by having a NiVertexColorProperty object as a property child of the root node. If this property object is not present, the vertex colors fine-tune lighting.\r\n\r\n            Note 2: set to either 0 or 0xFFFFFFFF for NifTexture compatibility.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Vertex Colors",
     *             type: "Color4",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             width: None,
     *             cond: Some(
     *                 "Has Vertex Colors",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "#VEC4_ONE#",
     *             ),
     *             description: Some(
     *                 "The vertex colors.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Data Flags",
     *             type: "NiGeometryDataFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "4.2.2.0",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The lower 6 bits of this field represent the number of UV texture sets. The rest is unused.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Has UV",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "4.0.0.2",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Do we have UV coordinates?\r\n\r\n            Note: for compatibility with NifTexture, set this value to either 0x00000000 or 0xFFFFFFFF.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "UV Sets",
     *             type: "TexCoord",
     *             length: Some(
     *                 "((Data Flags #BITAND# 63) #BITOR# (BS Data Flags #BITAND# 1))",
     *             ),
     *             width: Some(
     *                 "Num Vertices",
     *             ),
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The UV texture coordinates. They follow the OpenGL standard: some programs may require you to flip the second coordinate.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Consistency Flags",
     *             type: "ConsistencyType",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.0.1.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "CT_MUTABLE",
     *             ),
     *             description: Some(
     *                 "Consistency Flags",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Additional Data",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.0.0.4",
     *             ),
     *             until: None,
     *             template: Some(
     *                 "AbstractAdditionalGeometryData",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Mesh data: vertices, vertex normals, etc.
    /// Bethesda 20.2.0.7 NIFs: NiParticlesData no longer inherits from NiGeometryData and inherits NiObject directly.
    /// "Num Vertices" is renamed to "BS Max Vertices" for Bethesda 20.2 because Vertices, Normals, Tangents, Colors, and UV arrays
    /// do not have length for NiPSysData regardless of "Num" or booleans.
    #[binrw::binrw]
    pub struct NiGeometryData {
        pub base: NiObject,
        /// Always zero.
        pub group_id: i32,
        /// Number of vertices.
        pub num_vertices: u16,
        /// Number of vertices.
        pub num_vertices: u16,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub bs_max_vertices: u16,
        /// Used with NiCollision objects when OBB or TRI is set.
        pub keep_flags: u8,
        pub compress_flags: u8,
        /// Is the vertex array present? (Always non-zero.)
        pub has_vertices: u8,
        /// The mesh vertices.
        pub vertices: Vector3,
        pub data_flags: NiGeometryDataFlags,
        pub bs_data_flags: BSGeometryDataFlags,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub material_crc: u32,
        /// Do we have lighting normals? These are essential for proper lighting: if not present, the model will only be influenced by ambient light.
        pub has_normals: u8,
        /// The lighting normals.
        pub normals: Vector3,
        /// Tangent vectors.
        pub tangents: Vector3,
        /// Bitangent vectors.
        pub bitangents: Vector3,
        // field is out of version range (Some("20.3.0.9"), Some("20.3.0.9"))
        // pub has_div2_floats: u8,
        // field is out of version range (Some("20.3.0.9"), Some("20.3.0.9"))
        // pub div2_floats: f32,
        pub bounding_sphere: NiBound,
        /// Do we have vertex colors? These are usually used to fine-tune the lighting of the model.
        ///
        /// Note: how vertex colors influence the model can be controlled by having a NiVertexColorProperty object as a property child of the root node. If this property object is not present, the vertex colors fine-tune lighting.
        ///
        /// Note 2: set to either 0 or 0xFFFFFFFF for NifTexture compatibility.
        pub has_vertex_colors: u8,
        /// The vertex colors.
        pub vertex_colors: Color4,
        // field is out of version range (None, Some("4.2.2.0"))
        // pub data_flags: NiGeometryDataFlags,
        // field is out of version range (None, Some("4.0.0.2"))
        // pub has_uv: u8,
        /// The UV texture coordinates. They follow the OpenGL standard: some programs may require you to flip the second coordinate.
        pub uv_sets: TexCoord,
        /// Consistency Flags
        pub consistency_flags: ConsistencyType,
        pub additional_data: Ref<AbstractAdditionalGeometryData>,
    }
    /*
     * NiObject {
     *     name: "AbstractAdditionalGeometryData",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: None,
     *     fields: [],
     * }
     */
    #[binrw::binrw]
    pub struct AbstractAdditionalGeometryData {
        pub base: NiObject,
    }
    /*
     * NiObject {
     *     name: "NiTriBasedGeomData",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiGeometryData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Describes a mesh, built from triangles.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Num Triangles",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Number of triangles.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Describes a mesh, built from triangles.
    #[binrw::binrw]
    pub struct NiTriBasedGeomData {
        pub base: NiGeometryData,
        /// Number of triangles.
        pub num_triangles: u16,
    }
    /*
     * NiObject {
     *     name: "NiAlphaProperty",
     *     abstract: None,
     *     inherit: Some(
     *         "NiProperty",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Transparency. Flags 0x00ED.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Flags",
     *             type: "AlphaFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "4844",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Threshold",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "128",
     *             ),
     *             description: Some(
     *                 "Threshold for alpha testing",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Unknown Short 1",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "2.3",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Unknown Int 2",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "2.3",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Transparency. Flags 0x00ED.
    #[binrw::binrw]
    pub struct NiAlphaProperty {
        pub base: NiProperty,
        pub flags: AlphaFlags,
        /// Threshold for alpha testing
        pub threshold: u8,
        // field is out of version range (None, Some("2.3"))
        // pub unknown_short_1: u16,
        // field is out of version range (None, Some("2.3"))
        // pub unknown_int_2: u32,
    }
    /*
     * NiObject {
     *     name: "NiAmbientLight",
     *     abstract: None,
     *     inherit: Some(
     *         "NiLight",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Ambient light source.",
     *     ),
     *     fields: [],
     * }
     */
    /// Ambient light source.
    #[binrw::binrw]
    pub struct NiAmbientLight {
        pub base: NiLight,
    }
    /*
     * NiObject {
     *     name: "NiParticlesData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiGeometryData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Generic rotating particles data object.\r\n        Bethesda 20.2.0.7 NIFs: NiParticlesData no longer inherits from NiGeometryData and inherits NiObject directly.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Num Particles",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "4.0.0.2",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The maximum number of particles (matches the number of vertices).",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Particle Radius",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.0.1.0",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The particles' size.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Has Radii",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Is the particle size array present?",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Radii",
     *             type: "float",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             width: None,
     *             cond: Some(
     *                 "Has Radii",
     *             ),
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The individual particle sizes.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Active",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The number of active particles at the time the system was saved. This is also the number of valid entries in the following arrays.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Has Sizes",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Is the particle size array present?",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Sizes",
     *             type: "float",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             width: None,
     *             cond: Some(
     *                 "Has Sizes",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The individual particle sizes.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Has Rotations",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.0.1.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Is the particle rotation array present?",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Rotations",
     *             type: "Quaternion",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             width: None,
     *             cond: Some(
     *                 "Has Rotations",
     *             ),
     *             since: Some(
     *                 "10.0.1.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The individual particle rotations.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Has Rotation Angles",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.0.0.4",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Are the angles of rotation present?",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Rotation Angles",
     *             type: "float",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             width: None,
     *             cond: Some(
     *                 "Has Rotation Angles",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Angles of rotation",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Has Rotation Axes",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.0.0.4",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Are axes of rotation present?",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Rotation Axes",
     *             type: "Vector3",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             width: None,
     *             cond: Some(
     *                 "Has Rotation Axes",
     *             ),
     *             since: Some(
     *                 "20.0.0.4",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Axes of rotation.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Has Texture Indices",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num Subtexture Offsets",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "How many quads to use in BSPSysSubTexModifier for texture atlasing",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Subtexture Offsets",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "2,4,8,16,32,64 are potential values. If \"Has\" was no then this should be 256, which represents a 16x16 framed image, which is invalid",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Subtexture Offsets",
     *             type: "Vector4",
     *             length: Some(
     *                 "Num Subtexture Offsets",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Defines UV offsets",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Aspect Ratio",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Sets aspect ratio for Subtexture Offset UV quads",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Aspect Flags",
     *             type: "AspectFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Speed to Aspect Aspect 2",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Speed to Aspect Speed 1",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Speed to Aspect Speed 2",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Generic rotating particles data object.
    /// Bethesda 20.2.0.7 NIFs: NiParticlesData no longer inherits from NiGeometryData and inherits NiObject directly.
    #[binrw::binrw]
    pub struct NiParticlesData {
        pub base: NiGeometryData,
        // field is out of version range (None, Some("4.0.0.2"))
        // pub num_particles: u16,
        // field is out of version range (None, Some("10.0.1.0"))
        // pub particle_radius: f32,
        /// Is the particle size array present?
        pub has_radii: u8,
        /// The individual particle sizes.
        pub radii: f32,
        /// The number of active particles at the time the system was saved. This is also the number of valid entries in the following arrays.
        pub num_active: u16,
        /// Is the particle size array present?
        pub has_sizes: u8,
        /// The individual particle sizes.
        pub sizes: f32,
        /// Is the particle rotation array present?
        pub has_rotations: u8,
        /// The individual particle rotations.
        pub rotations: Quaternion,
        /// Are the angles of rotation present?
        pub has_rotation_angles: u8,
        /// Angles of rotation
        pub rotation_angles: f32,
        /// Are axes of rotation present?
        pub has_rotation_axes: u8,
        /// Axes of rotation.
        pub rotation_axes: Vector3,
        pub has_texture_indices: u8,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub num_subtexture_offsets: u32,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub num_subtexture_offsets: u8,
        /// Defines UV offsets
        pub subtexture_offsets: Vector4,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub aspect_ratio: f32,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub aspect_flags: AspectFlags,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub speed_to_aspect_aspect_2: f32,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub speed_to_aspect_speed_1: f32,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub speed_to_aspect_speed_2: f32,
    }
    /*
     * NiObject {
     *     name: "NiBinaryExtraData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiExtraData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Binary extra data object. Used to store tangents and bitangents in Oblivion.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Binary Data",
     *             type: "ByteArray",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The binary data.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Binary extra data object. Used to store tangents and bitangents in Oblivion.
    #[binrw::binrw]
    pub struct NiBinaryExtraData {
        pub base: NiExtraData,
        /// The binary data.
        pub binary_data: ByteArray,
    }
    /*
     * NiObject {
     *     name: "NiBooleanExtraData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiExtraData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Boolean extra data.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Boolean Data",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The boolean extra data value.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Boolean extra data.
    #[binrw::binrw]
    pub struct NiBooleanExtraData {
        pub base: NiExtraData,
        /// The boolean extra data value.
        pub boolean_data: u8,
    }
    /*
     * NiObject {
     *     name: "NiCamera",
     *     abstract: None,
     *     inherit: Some(
     *         "NiAVObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Camera object.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Camera Flags",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Obsolete flags.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Frustum Left",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "-0.63707",
     *             ),
     *             description: Some(
     *                 "Frustrum left.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Frustum Right",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "0.63707",
     *             ),
     *             description: Some(
     *                 "Frustrum right.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Frustum Top",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "0.385714",
     *             ),
     *             description: Some(
     *                 "Frustrum top.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Frustum Bottom",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "-0.385714",
     *             ),
     *             description: Some(
     *                 "Frustrum bottom.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Frustum Near",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             description: Some(
     *                 "Frustrum near.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Frustum Far",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "5000.0",
     *             ),
     *             description: Some(
     *                 "Frustrum far.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Use Orthographic Projection",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Determines whether perspective is used.  Orthographic means no perspective.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Viewport Left",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Viewport left.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Viewport Right",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             description: Some(
     *                 "Viewport right.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Viewport Top",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             description: Some(
     *                 "Viewport top.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Viewport Bottom",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Viewport bottom.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "LOD Adjust",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             description: Some(
     *                 "Level of detail adjust.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Scene",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiAVObject",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num Screen Polygons",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Deprecated. Array is always zero length on disk write.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Screen Textures",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "4.2.1.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Deprecated. Array is always zero length on disk write.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Unknown Int 3",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "3.1",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Camera object.
    #[binrw::binrw]
    pub struct NiCamera {
        pub base: NiAVObject,
        /// Obsolete flags.
        pub camera_flags: u16,
        /// Frustrum left.
        pub frustum_left: f32,
        /// Frustrum right.
        pub frustum_right: f32,
        /// Frustrum top.
        pub frustum_top: f32,
        /// Frustrum bottom.
        pub frustum_bottom: f32,
        /// Frustrum near.
        pub frustum_near: f32,
        /// Frustrum far.
        pub frustum_far: f32,
        /// Determines whether perspective is used.  Orthographic means no perspective.
        pub use_orthographic_projection: u8,
        /// Viewport left.
        pub viewport_left: f32,
        /// Viewport right.
        pub viewport_right: f32,
        /// Viewport top.
        pub viewport_top: f32,
        /// Viewport bottom.
        pub viewport_bottom: f32,
        /// Level of detail adjust.
        pub lod_adjust: f32,
        pub scene: Ref<NiAVObject>,
        /// Deprecated. Array is always zero length on disk write.
        pub num_screen_polygons: u32,
        /// Deprecated. Array is always zero length on disk write.
        pub num_screen_textures: u32,
        // field is out of version range (None, Some("3.1"))
        // pub unknown_int_3: u32,
    }
    /*
     * NiObject {
     *     name: "NiColorExtraData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiExtraData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Extra data in the form of NiColorA (red, green, blue, alpha).",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Data",
     *             type: "Color4",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "RGBA Color?",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Extra data in the form of NiColorA (red, green, blue, alpha).
    #[binrw::binrw]
    pub struct NiColorExtraData {
        pub base: NiExtraData,
        /// RGBA Color?
        pub data: Color4,
    }
    /*
     * NiObject {
     *     name: "NiAVObjectPalette",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Abstract base class for indexing NiAVObject by name.",
     *     ),
     *     fields: [],
     * }
     */
    /// Abstract base class for indexing NiAVObject by name.
    #[binrw::binrw]
    pub struct NiAVObjectPalette {
        pub base: NiObject,
    }
    /*
     * NiObject {
     *     name: "NiDefaultAVObjectPalette",
     *     abstract: None,
     *     inherit: Some(
     *         "NiAVObjectPalette",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "NiAVObjectPalette implementation. Used to quickly look up objects by name.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Scene",
     *             type: "Ptr",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiAVObject",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Scene root of the object palette.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Objs",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Number of objects.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Objs",
     *             type: "AVObject",
     *             length: Some(
     *                 "Num Objs",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The objects.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// NiAVObjectPalette implementation. Used to quickly look up objects by name.
    #[binrw::binrw]
    pub struct NiDefaultAVObjectPalette {
        pub base: NiAVObjectPalette,
        /// Scene root of the object palette.
        pub scene: Ptr<NiAVObject>,
        /// Number of objects.
        pub num_objs: u32,
        /// The objects.
        pub objs: AVObject,
    }
    /*
     * NiObject {
     *     name: "NiDirectionalLight",
     *     abstract: None,
     *     inherit: Some(
     *         "NiLight",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Directional light source.",
     *     ),
     *     fields: [],
     * }
     */
    /// Directional light source.
    #[binrw::binrw]
    pub struct NiDirectionalLight {
        pub base: NiLight,
    }
    /*
     * NiObject {
     *     name: "NiDitherProperty",
     *     abstract: None,
     *     inherit: Some(
     *         "NiProperty",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "NiDitherProperty allows the application to turn the dithering of interpolated colors and fog values on and off.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Flags",
     *             type: "DitherFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// NiDitherProperty allows the application to turn the dithering of interpolated colors and fog values on and off.
    #[binrw::binrw]
    pub struct NiDitherProperty {
        pub base: NiProperty,
        pub flags: DitherFlags,
    }
    /*
     * NiObject {
     *     name: "NiFloatExtraData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiExtraData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Extra float data.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Float Data",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The float data.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Extra float data.
    #[binrw::binrw]
    pub struct NiFloatExtraData {
        pub base: NiExtraData,
        /// The float data.
        pub float_data: f32,
    }
    /*
     * NiObject {
     *     name: "NiFloatsExtraData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiExtraData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Extra float array data.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Num Floats",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Number of floats in the next field.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Data",
     *             type: "float",
     *             length: Some(
     *                 "Num Floats",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Float data.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Extra float array data.
    #[binrw::binrw]
    pub struct NiFloatsExtraData {
        pub base: NiExtraData,
        /// Number of floats in the next field.
        pub num_floats: u32,
        /// Float data.
        pub data: f32,
    }
    /*
     * NiObject {
     *     name: "NiFogProperty",
     *     abstract: None,
     *     inherit: Some(
     *         "NiProperty",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "NiFogProperty allows the application to enable, disable and control the appearance of fog.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Flags",
     *             type: "FogFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Fog Depth",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             description: Some(
     *                 "Depth of the fog in normalized units. 1.0 = begins at near plane. 0.5 = begins halfway between the near and far planes.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Fog Color",
     *             type: "Color3",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "#VEC3_ZERO#",
     *             ),
     *             description: Some(
     *                 "The color of the fog.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// NiFogProperty allows the application to enable, disable and control the appearance of fog.
    #[binrw::binrw]
    pub struct NiFogProperty {
        pub base: NiProperty,
        pub flags: FogFlags,
        /// Depth of the fog in normalized units. 1.0 = begins at near plane. 0.5 = begins halfway between the near and far planes.
        pub fog_depth: f32,
        /// The color of the fog.
        pub fog_color: Color3,
    }
    /*
     * NiObject {
     *     name: "NiIntegerExtraData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiExtraData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Extra integer data.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Integer Data",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The value of the extra data.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Extra integer data.
    #[binrw::binrw]
    pub struct NiIntegerExtraData {
        pub base: NiExtraData,
        /// The value of the extra data.
        pub integer_data: u32,
    }
    /*
     * NiObject {
     *     name: "NiIntegersExtraData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiExtraData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Extra integer array data.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Num Integers",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Number of integers.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Data",
     *             type: "uint",
     *             length: Some(
     *                 "Num Integers",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Integers.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Extra integer array data.
    #[binrw::binrw]
    pub struct NiIntegersExtraData {
        pub base: NiExtraData,
        /// Number of integers.
        pub num_integers: u32,
        /// Integers.
        pub data: u32,
    }
    /*
     * NiObject {
     *     name: "NiMaterialProperty",
     *     abstract: None,
     *     inherit: Some(
     *         "NiProperty",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Describes the surface properties of an object e.g. translucency, ambient color, diffuse color, emissive color, and specular color.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Flags",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "3.0",
     *             ),
     *             until: Some(
     *                 "10.0.1.2",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Property flags.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Ambient Color",
     *             type: "Color3",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "#VEC3_ONE#",
     *             ),
     *             description: Some(
     *                 "How much the material reflects ambient light.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Diffuse Color",
     *             type: "Color3",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "#VEC3_ONE#",
     *             ),
     *             description: Some(
     *                 "How much the material reflects diffuse light.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Specular Color",
     *             type: "Color3",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "#VEC3_ONE#",
     *             ),
     *             description: Some(
     *                 "How much light the material reflects in a specular manner.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Emissive Color",
     *             type: "Color3",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "#VEC3_ZERO#",
     *             ),
     *             description: Some(
     *                 "How much light the material emits.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Glossiness",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "10.0",
     *             ),
     *             description: Some(
     *                 "The material glossiness.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Alpha",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             description: Some(
     *                 "The material transparency (1=non-transparant). Refer to a NiAlphaProperty object in this material's parent NiTriShape object, when alpha is not 1.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Emissive Mult",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Describes the surface properties of an object e.g. translucency, ambient color, diffuse color, emissive color, and specular color.
    #[binrw::binrw]
    pub struct NiMaterialProperty {
        pub base: NiProperty,
        // field is out of version range (Some("3.0"), Some("10.0.1.2"))
        // pub flags: u16,
        /// How much the material reflects ambient light.
        pub ambient_color: Color3,
        /// How much the material reflects diffuse light.
        pub diffuse_color: Color3,
        /// How much light the material reflects in a specular manner.
        pub specular_color: Color3,
        /// How much light the material emits.
        pub emissive_color: Color3,
        /// The material glossiness.
        pub glossiness: f32,
        /// The material transparency (1=non-transparant). Refer to a NiAlphaProperty object in this material's parent NiTriShape object, when alpha is not 1.
        pub alpha: f32,
        pub emissive_mult: f32,
    }
    /*
     * NiObject {
     *     name: "NiNode",
     *     abstract: None,
     *     inherit: Some(
     *         "NiAVObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Generic node object for grouping.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Num Children",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The number of child objects.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Children",
     *             type: "Ref",
     *             length: Some(
     *                 "Num Children",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiAVObject",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "List of child node object indices.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Effects",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The number of references to effect objects that follow.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Effects",
     *             type: "Ref",
     *             length: Some(
     *                 "Num Effects",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiDynamicEffect",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "List of node effects. ADynamicEffect?",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Generic node object for grouping.
    #[binrw::binrw]
    pub struct NiNode {
        pub base: NiAVObject,
        /// The number of child objects.
        pub num_children: u32,
        /// List of child node object indices.
        pub children: Ref<NiAVObject>,
        /// The number of references to effect objects that follow.
        pub num_effects: u32,
        /// List of node effects. ADynamicEffect?
        pub effects: Ref<NiDynamicEffect>,
    }
    /*
     * NiObject {
     *     name: "NiBillboardNode",
     *     abstract: None,
     *     inherit: Some(
     *         "NiNode",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "These nodes will always be rotated to face the camera creating a billboard effect for any attached objects.\r\n\r\n        In pre-10.1.0.0 the Flags field is used for BillboardMode.\r\n        Bit 0: hidden\r\n        Bits 1-2: collision mode\r\n        Bit 3: unknown (set in most official meshes)\r\n        Bits 5-6: billboard mode\r\n\r\n        Collision modes:\r\n        00 NONE\r\n        01 USE_TRIANGLES\r\n        10 USE_OBBS\r\n        11 CONTINUE\r\n\r\n        Billboard modes:\r\n        00 ALWAYS_FACE_CAMERA\r\n        01 ROTATE_ABOUT_UP\r\n        10 RIGID_FACE_CAMERA\r\n        11 ALWAYS_FACE_CENTER",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Billboard Mode",
     *             type: "BillboardMode",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The way the billboard will react to the camera.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// These nodes will always be rotated to face the camera creating a billboard effect for any attached objects.
    ///
    /// In pre-10.1.0.0 the Flags field is used for BillboardMode.
    /// Bit 0: hidden
    /// Bits 1-2: collision mode
    /// Bit 3: unknown (set in most official meshes)
    /// Bits 5-6: billboard mode
    ///
    /// Collision modes:
    /// 00 NONE
    /// 01 USE_TRIANGLES
    /// 10 USE_OBBS
    /// 11 CONTINUE
    ///
    /// Billboard modes:
    /// 00 ALWAYS_FACE_CAMERA
    /// 01 ROTATE_ABOUT_UP
    /// 10 RIGID_FACE_CAMERA
    /// 11 ALWAYS_FACE_CENTER
    #[binrw::binrw]
    pub struct NiBillboardNode {
        pub base: NiNode,
        /// The way the billboard will react to the camera.
        pub billboard_mode: BillboardMode,
    }
    /*
     * NiObject {
     *     name: "NiSwitchNode",
     *     abstract: None,
     *     inherit: Some(
     *         "NiNode",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Represents groups of multiple scenegraph subtrees, only one of which (the \"active child\") is drawn at any given time.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Switch Node Flags",
     *             type: "NiSwitchFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "3",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Index",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Represents groups of multiple scenegraph subtrees, only one of which (the "active child") is drawn at any given time.
    #[binrw::binrw]
    pub struct NiSwitchNode {
        pub base: NiNode,
        pub switch_node_flags: NiSwitchFlags,
        pub index: u32,
    }
    /*
     * NiObject {
     *     name: "NiLODNode",
     *     abstract: None,
     *     inherit: Some(
     *         "NiSwitchNode",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Level of detail selector. Links to different levels of detail of the same model, used to switch a geometry at a specified distance.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "LOD Center",
     *             type: "Vector3",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "4.0.0.2",
     *             ),
     *             until: Some(
     *                 "10.0.1.0",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num LOD Levels",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.0.1.0",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "LOD Levels",
     *             type: "LODRange",
     *             length: Some(
     *                 "Num LOD Levels",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.0.1.0",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "LOD Level Data",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: Some(
     *                 "NiLODData",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Level of detail selector. Links to different levels of detail of the same model, used to switch a geometry at a specified distance.
    #[binrw::binrw]
    pub struct NiLODNode {
        pub base: NiSwitchNode,
        // field is out of version range (Some("4.0.0.2"), Some("10.0.1.0"))
        // pub lod_center: Vector3,
        // field is out of version range (None, Some("10.0.1.0"))
        // pub num_lod_levels: u32,
        // field is out of version range (None, Some("10.0.1.0"))
        // pub lod_levels: LODRange,
        pub lod_level_data: Ref<NiLODData>,
    }
    /*
     * NiObject {
     *     name: "NiPalette",
     *     abstract: None,
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "NiPalette objects represent mappings from 8-bit indices to 24-bit RGB or 32-bit RGBA colors.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Has Alpha",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Not boolean but used as one, always 8-bit.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Entries",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "256",
     *             ),
     *             description: Some(
     *                 "The number of palette entries. Always 256 but can also be 16.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Palette",
     *             type: "ByteColor4",
     *             length: Some(
     *                 "16",
     *             ),
     *             width: None,
     *             cond: Some(
     *                 "Num Entries == 16",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The color palette.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Palette",
     *             type: "ByteColor4",
     *             length: Some(
     *                 "256",
     *             ),
     *             width: None,
     *             cond: Some(
     *                 "Num Entries != 16",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The color palette.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// NiPalette objects represent mappings from 8-bit indices to 24-bit RGB or 32-bit RGBA colors.
    #[binrw::binrw]
    pub struct NiPalette {
        pub base: NiObject,
        /// Not boolean but used as one, always 8-bit.
        pub has_alpha: u8,
        /// The number of palette entries. Always 256 but can also be 16.
        pub num_entries: u32,
        /// The color palette.
        pub palette: ByteColor4,
        /// The color palette.
        pub palette: ByteColor4,
    }
    /*
     * NiObject {
     *     name: "NiParticles",
     *     abstract: None,
     *     inherit: Some(
     *         "NiGeometry",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Generic particle system node.",
     *     ),
     *     fields: [],
     * }
     */
    /// Generic particle system node.
    #[binrw::binrw]
    pub struct NiParticles {
        pub base: NiGeometry,
    }
    /*
     * NiObject {
     *     name: "NiPixelFormat",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "NiPixelFormat is not the parent to NiPixelData/NiPersistentSrcTextureRendererData,\r\n        but actually a member class loaded at the top of each. The two classes are not related.\r\n        However, faking this inheritance is useful for several things.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Pixel Format",
     *             type: "PixelFormat",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The format of the pixels in this internally stored image.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Red Mask",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.4.0.1",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "0x000000ff (for 24bpp and 32bpp) or 0x00000000 (for 8bpp)",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Green Mask",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.4.0.1",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "0x0000ff00 (for 24bpp and 32bpp) or 0x00000000 (for 8bpp)",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Blue Mask",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.4.0.1",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "0x00ff0000 (for 24bpp and 32bpp) or 0x00000000 (for 8bpp)",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Alpha Mask",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.4.0.1",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "0xff000000 (for 32bpp) or 0x00000000 (for 24bpp and 8bpp)",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Bits Per Pixel",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.4.0.1",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Bits per pixel, 0 (Compressed), 8, 24 or 32.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Old Fast Compare",
     *             type: "byte",
     *             length: Some(
     *                 "8",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.4.0.1",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "[96,8,130,0,0,65,0,0] if 24 bits per pixel\r\n            [129,8,130,32,0,65,12,0] if 32 bits per pixel\r\n            [34,0,0,0,0,0,0,0] if 8 bits per pixel\r\n            [X,0,0,0,0,0,0,0] if 0 (Compressed) bits per pixel where X = PixelFormat",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Tiling",
     *             type: "PixelTiling",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: Some(
     *                 "10.4.0.1",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Seems to always be zero.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Bits Per Pixel",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.4.0.2",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Bits per pixel, 0 (Compressed), 8, 24 or 32.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Renderer Hint",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.4.0.2",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Extra Data",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.4.0.2",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Flags",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.4.0.2",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Tiling",
     *             type: "PixelTiling",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.4.0.2",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "sRGB Space",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.3.0.4",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Channels",
     *             type: "PixelFormatComponent",
     *             length: Some(
     *                 "4",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.4.0.2",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Channel Data",
     *             ),
     *         },
     *     ],
     * }
     */
    /// NiPixelFormat is not the parent to NiPixelData/NiPersistentSrcTextureRendererData,
    /// but actually a member class loaded at the top of each. The two classes are not related.
    /// However, faking this inheritance is useful for several things.
    #[binrw::binrw]
    pub struct NiPixelFormat {
        pub base: NiObject,
        /// The format of the pixels in this internally stored image.
        pub pixel_format: PixelFormat,
        // field is out of version range (None, Some("10.4.0.1"))
        // pub red_mask: u32,
        // field is out of version range (None, Some("10.4.0.1"))
        // pub green_mask: u32,
        // field is out of version range (None, Some("10.4.0.1"))
        // pub blue_mask: u32,
        // field is out of version range (None, Some("10.4.0.1"))
        // pub alpha_mask: u32,
        // field is out of version range (None, Some("10.4.0.1"))
        // pub bits_per_pixel: u32,
        // field is out of version range (None, Some("10.4.0.1"))
        // pub old_fast_compare: u8,
        // field is out of version range (Some("10.1.0.0"), Some("10.4.0.1"))
        // pub tiling: PixelTiling,
        /// Bits per pixel, 0 (Compressed), 8, 24 or 32.
        pub bits_per_pixel: u8,
        pub renderer_hint: u32,
        pub extra_data: u32,
        pub flags: u8,
        pub tiling: PixelTiling,
        // field is out of version range (Some("20.3.0.4"), None)
        // pub srgb_space: u8,
        /// Channel Data
        pub channels: PixelFormatComponent,
    }
    /*
     * NiObject {
     *     name: "NiPersistentSrcTextureRendererData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiPixelFormat",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: None,
     *     fields: [
     *         NiObjectField {
     *             name: "Palette",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiPalette",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num Mipmaps",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Bytes Per Pixel",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Mipmaps",
     *             type: "MipMap",
     *             length: Some(
     *                 "Num Mipmaps",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num Pixels",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Pad Num Pixels",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.6",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num Faces",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Platform",
     *             type: "PlatformID",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "30.1.0.0",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Renderer",
     *             type: "RendererID",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "30.1.0.1",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Pixel Data",
     *             type: "byte",
     *             length: Some(
     *                 "Num Pixels #MUL# Num Faces",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    #[binrw::binrw]
    pub struct NiPersistentSrcTextureRendererData {
        pub base: NiPixelFormat,
        pub palette: Ref<NiPalette>,
        pub num_mipmaps: u32,
        pub bytes_per_pixel: u32,
        pub mipmaps: MipMap,
        pub num_pixels: u32,
        // field is out of version range (Some("20.2.0.6"), None)
        // pub pad_num_pixels: u32,
        pub num_faces: u32,
        pub platform: PlatformID,
        // field is out of version range (Some("30.1.0.1"), None)
        // pub renderer: RendererID,
        pub pixel_data: u8,
    }
    /*
     * NiObject {
     *     name: "NiPixelData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiPixelFormat",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "A texture.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Palette",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiPalette",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num Mipmaps",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Bytes Per Pixel",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Mipmaps",
     *             type: "MipMap",
     *             length: Some(
     *                 "Num Mipmaps",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num Pixels",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num Faces",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.4.0.2",
     *             ),
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Pixel Data",
     *             type: "byte",
     *             length: Some(
     *                 "Num Pixels",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.4.0.1",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Pixel Data",
     *             type: "byte",
     *             length: Some(
     *                 "Num Pixels #MUL# Num Faces",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.4.0.2",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// A texture.
    #[binrw::binrw]
    pub struct NiPixelData {
        pub base: NiPixelFormat,
        pub palette: Ref<NiPalette>,
        pub num_mipmaps: u32,
        pub bytes_per_pixel: u32,
        pub mipmaps: MipMap,
        pub num_pixels: u32,
        pub num_faces: u32,
        // field is out of version range (None, Some("10.4.0.1"))
        // pub pixel_data: u8,
        pub pixel_data: u8,
    }
    /*
     * NiObject {
     *     name: "NiPointLight",
     *     abstract: None,
     *     inherit: Some(
     *         "NiLight",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "A point light.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Constant Attenuation",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Linear Attenuation",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Quadratic Attenuation",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// A point light.
    #[binrw::binrw]
    pub struct NiPointLight {
        pub base: NiLight,
        pub constant_attenuation: f32,
        pub linear_attenuation: f32,
        pub quadratic_attenuation: f32,
    }
    /*
     * NiObject {
     *     name: "NiLODData",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Abstract class used for different types of LOD selections.",
     *     ),
     *     fields: [],
     * }
     */
    /// Abstract class used for different types of LOD selections.
    #[binrw::binrw]
    pub struct NiLODData {
        pub base: NiObject,
    }
    /*
     * NiObject {
     *     name: "NiRangeLODData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiLODData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "NiRangeLODData controls switching LOD levels based on Z depth from the camera to the NiLODNode.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "LOD Center",
     *             type: "Vector3",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num LOD Levels",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "LOD Levels",
     *             type: "LODRange",
     *             length: Some(
     *                 "Num LOD Levels",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// NiRangeLODData controls switching LOD levels based on Z depth from the camera to the NiLODNode.
    #[binrw::binrw]
    pub struct NiRangeLODData {
        pub base: NiLODData,
        pub lod_center: Vector3,
        pub num_lod_levels: u32,
        pub lod_levels: LODRange,
    }
    /*
     * NiObject {
     *     name: "NiScreenLODData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiLODData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "NiScreenLODData controls switching LOD levels based on proportion of the screen that a bound would include.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Bounding Sphere",
     *             type: "NiBound",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "World Bounding Sphere",
     *             type: "NiBound",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num Proportions",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Proportion Levels",
     *             type: "float",
     *             length: Some(
     *                 "Num Proportions",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// NiScreenLODData controls switching LOD levels based on proportion of the screen that a bound would include.
    #[binrw::binrw]
    pub struct NiScreenLODData {
        pub base: NiLODData,
        pub bounding_sphere: NiBound,
        pub world_bounding_sphere: NiBound,
        pub num_proportions: u32,
        pub proportion_levels: f32,
    }
    /*
     * NiObject {
     *     name: "NiShadeProperty",
     *     abstract: None,
     *     inherit: Some(
     *         "NiProperty",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Determines whether flat shading or smooth shading is used on a shape.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Flags",
     *             type: "ShadeFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "SHADING_SMOOTH",
     *             ),
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Determines whether flat shading or smooth shading is used on a shape.
    #[binrw::binrw]
    pub struct NiShadeProperty {
        pub base: NiProperty,
        pub flags: ShadeFlags,
    }
    /*
     * NiObject {
     *     name: "NiSkinData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Skinning data.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Skin Transform",
     *             type: "NiTransform",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Offset of the skin from this bone in bind position.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Bones",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Number of bones.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Skin Partition",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "4.0.0.2",
     *             ),
     *             until: Some(
     *                 "10.1.0.0",
     *             ),
     *             template: Some(
     *                 "NiSkinPartition",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "This optionally links a NiSkinPartition for hardware-acceleration information.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Has Vertex Weights",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "4.2.1.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "true",
     *             ),
     *             description: Some(
     *                 "Enables Vertex Weights for this NiSkinData.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Bone List",
     *             type: "BoneData",
     *             length: Some(
     *                 "Num Bones",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Contains offset data for each node that this skin is influenced by.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Skinning data.
    #[binrw::binrw]
    pub struct NiSkinData {
        pub base: NiObject,
        /// Offset of the skin from this bone in bind position.
        pub skin_transform: NiTransform,
        /// Number of bones.
        pub num_bones: u32,
        // field is out of version range (Some("4.0.0.2"), Some("10.1.0.0"))
        // pub skin_partition: Ref<NiSkinPartition>,
        /// Enables Vertex Weights for this NiSkinData.
        pub has_vertex_weights: u8,
        /// Contains offset data for each node that this skin is influenced by.
        pub bone_list: BoneData,
    }
    /*
     * NiObject {
     *     name: "NiSkinInstance",
     *     abstract: None,
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Skinning instance.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Data",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiSkinData",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Skinning data reference.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Skin Partition",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.101",
     *             ),
     *             until: None,
     *             template: Some(
     *                 "NiSkinPartition",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Refers to a NiSkinPartition objects, which partitions the mesh such that every vertex is only influenced by a limited number of bones.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Skeleton Root",
     *             type: "Ptr",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiNode",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Armature root node.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Bones",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The number of node bones referenced as influences.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Bones",
     *             type: "Ptr",
     *             length: Some(
     *                 "Num Bones",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiNode",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "List of all armature bones.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Skinning instance.
    #[binrw::binrw]
    pub struct NiSkinInstance {
        pub base: NiObject,
        /// Skinning data reference.
        pub data: Ref<NiSkinData>,
        /// Refers to a NiSkinPartition objects, which partitions the mesh such that every vertex is only influenced by a limited number of bones.
        pub skin_partition: Ref<NiSkinPartition>,
        /// Armature root node.
        pub skeleton_root: Ptr<NiNode>,
        /// The number of node bones referenced as influences.
        pub num_bones: u32,
        /// List of all armature bones.
        pub bones: Ptr<NiNode>,
    }
    /*
     * NiObject {
     *     name: "NiSkinPartition",
     *     abstract: None,
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Skinning data, optimized for hardware skinning. The mesh is partitioned in submeshes such that each vertex of a submesh is influenced only by a limited and fixed number of bones.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Num Partitions",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Data Size",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Vertex Size",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Vertex Desc",
     *             type: "BSVertexDesc",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Vertex Data",
     *             type: "BSVertexDataSSE",
     *             length: Some(
     *                 "Data Size / Vertex Size",
     *             ),
     *             width: None,
     *             cond: Some(
     *                 "Data Size #GT# 0",
     *             ),
     *             since: Some(
     *                 "20.2.0.7",
     *             ),
     *             until: Some(
     *                 "20.2.0.7",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Partitions",
     *             type: "SkinPartition",
     *             length: Some(
     *                 "Num Partitions",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Skinning data, optimized for hardware skinning. The mesh is partitioned in submeshes such that each vertex of a submesh is influenced only by a limited and fixed number of bones.
    #[binrw::binrw]
    pub struct NiSkinPartition {
        pub base: NiObject,
        pub num_partitions: u32,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub data_size: u32,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub vertex_size: u32,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub vertex_desc: BSVertexDesc,
        // field is out of version range (Some("20.2.0.7"), Some("20.2.0.7"))
        // pub vertex_data: BSVertexDataSSE,
        pub partitions: SkinPartition,
    }
    /*
     * NiObject {
     *     name: "NiTexture",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiObjectNET",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "A texture.",
     *     ),
     *     fields: [],
     * }
     */
    /// A texture.
    #[binrw::binrw]
    pub struct NiTexture {
        pub base: NiObjectNET,
    }
    /*
     * NiObject {
     *     name: "NiSourceTexture",
     *     abstract: None,
     *     inherit: Some(
     *         "NiTexture",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Describes texture source and properties.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Use External",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1",
     *             ),
     *             description: Some(
     *                 "Is the texture external?",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Use Internal",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Use External == 0",
     *             ),
     *             since: None,
     *             until: Some(
     *                 "10.0.1.3",
     *             ),
     *             template: None,
     *             default: Some(
     *                 "1",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "File Name",
     *             type: "FilePath",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Use External == 1",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The external texture file name.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "File Name",
     *             type: "FilePath",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Use External == 0",
     *             ),
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The original source filename of the image embedded by the referred NiPixelData object.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Pixel Data",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Use External == 1",
     *             ),
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: Some(
     *                 "NiPixelFormat",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Pixel Data",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Use External == 0 #AND# Use Internal == 1",
     *             ),
     *             since: None,
     *             until: Some(
     *                 "10.0.1.3",
     *             ),
     *             template: Some(
     *                 "NiPixelFormat",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "NiPixelData or NiPersistentSrcTextureRendererData",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Pixel Data",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Use External == 0",
     *             ),
     *             since: Some(
     *                 "10.0.1.4",
     *             ),
     *             until: None,
     *             template: Some(
     *                 "NiPixelFormat",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "NiPixelData or NiPersistentSrcTextureRendererData",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Format Prefs",
     *             type: "FormatPrefs",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "A set of preferences for the texture format. They are a request only and the renderer may ignore them.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Is Static",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1",
     *             ),
     *             description: Some(
     *                 "If set, then the application cannot assume that any dynamic changes to the pixel data will show in the rendered image.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Direct Render",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.103",
     *             ),
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "true",
     *             ),
     *             description: Some(
     *                 "A hint to the renderer that the texture can be loaded directly from a texture file into a renderer-specific resource, bypassing the NiPixelData object.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Persist Render Data",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.4",
     *             ),
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "false",
     *             ),
     *             description: Some(
     *                 "Pixel Data is NiPersistentSrcTextureRendererData instead of NiPixelData.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Describes texture source and properties.
    #[binrw::binrw]
    pub struct NiSourceTexture {
        pub base: NiTexture,
        /// Is the texture external?
        pub use_external: u8,
        // field is out of version range (None, Some("10.0.1.3"))
        // pub use_internal: u8,
        /// The external texture file name.
        pub file_name: FilePath,
        /// The original source filename of the image embedded by the referred NiPixelData object.
        pub file_name: FilePath,
        pub pixel_data: Ref<NiPixelFormat>,
        // field is out of version range (None, Some("10.0.1.3"))
        // pub pixel_data: Ref<NiPixelFormat>,
        /// NiPixelData or NiPersistentSrcTextureRendererData
        pub pixel_data: Ref<NiPixelFormat>,
        /// A set of preferences for the texture format. They are a request only and the renderer may ignore them.
        pub format_prefs: FormatPrefs,
        /// If set, then the application cannot assume that any dynamic changes to the pixel data will show in the rendered image.
        pub is_static: u8,
        /// A hint to the renderer that the texture can be loaded directly from a texture file into a renderer-specific resource, bypassing the NiPixelData object.
        pub direct_render: u8,
        // field is out of version range (Some("20.2.0.4"), None)
        // pub persist_render_data: u8,
    }
    /*
     * NiObject {
     *     name: "NiSpecularProperty",
     *     abstract: None,
     *     inherit: Some(
     *         "NiProperty",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Gives specularity to a shape. Flags 0x0001.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Flags",
     *             type: "SpecularFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Gives specularity to a shape. Flags 0x0001.
    #[binrw::binrw]
    pub struct NiSpecularProperty {
        pub base: NiProperty,
        pub flags: SpecularFlags,
    }
    /*
     * NiObject {
     *     name: "NiSpotLight",
     *     abstract: None,
     *     inherit: Some(
     *         "NiPointLight",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "A spot.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Outer Spot Angle",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Inner Spot Angle",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.2.0.5",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Exponent",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1.0",
     *             ),
     *             description: Some(
     *                 "Describes the distribution of light.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// A spot.
    #[binrw::binrw]
    pub struct NiSpotLight {
        pub base: NiPointLight,
        pub outer_spot_angle: f32,
        // field is out of version range (Some("20.2.0.5"), None)
        // pub inner_spot_angle: f32,
        /// Describes the distribution of light.
        pub exponent: f32,
    }
    /*
     * NiObject {
     *     name: "NiStencilProperty",
     *     abstract: None,
     *     inherit: Some(
     *         "NiProperty",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Allows control of stencil testing.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Flags",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.0.1.2",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Property flags.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Stencil Enabled",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Enables or disables the stencil test.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Stencil Function",
     *             type: "StencilTestFunc",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Selects the compare mode function.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Stencil Ref",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Stencil Mask",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             template: None,
     *             default: Some(
     *                 "#UINT_MAX#",
     *             ),
     *             description: Some(
     *                 "A bit mask. The default is 0xffffffff.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Fail Action",
     *             type: "StencilAction",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Z Fail Action",
     *             type: "StencilAction",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Pass Action",
     *             type: "StencilAction",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Draw Mode",
     *             type: "StencilDrawMode",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             template: None,
     *             default: Some(
     *                 "DRAW_BOTH",
     *             ),
     *             description: Some(
     *                 "Used to enabled double sided faces. Default is 3 (DRAW_BOTH).",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Flags",
     *             type: "StencilFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.1.0.3",
     *             ),
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "19840",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Stencil Ref",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.1.0.3",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Stencil Mask",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.1.0.3",
     *             ),
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "#UINT_MAX#",
     *             ),
     *             description: Some(
     *                 "A bit mask. The default is 0xffffffff.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Allows control of stencil testing.
    #[binrw::binrw]
    pub struct NiStencilProperty {
        pub base: NiProperty,
        // field is out of version range (None, Some("10.0.1.2"))
        // pub flags: u16,
        /// Enables or disables the stencil test.
        pub stencil_enabled: u8,
        /// Selects the compare mode function.
        pub stencil_function: StencilTestFunc,
        pub stencil_ref: u32,
        /// A bit mask. The default is 0xffffffff.
        pub stencil_mask: u32,
        pub fail_action: StencilAction,
        pub z_fail_action: StencilAction,
        pub pass_action: StencilAction,
        /// Used to enabled double sided faces. Default is 3 (DRAW_BOTH).
        pub draw_mode: StencilDrawMode,
        // field is out of version range (Some("20.1.0.3"), None)
        // pub flags: StencilFlags,
        // field is out of version range (Some("20.1.0.3"), None)
        // pub stencil_ref: u32,
        // field is out of version range (Some("20.1.0.3"), None)
        // pub stencil_mask: u32,
    }
    /*
     * NiObject {
     *     name: "NiStringExtraData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiExtraData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Extra data in the form of text.\r\n        Used in various official or user-defined ways, e.g. preventing optimization on objects (\"NiOptimizeKeep\", \"sgoKeep\").",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "String Data",
     *             type: "string",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "4.0.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The string.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Extra data in the form of text.
    /// Used in various official or user-defined ways, e.g. preventing optimization on objects ("NiOptimizeKeep", "sgoKeep").
    #[binrw::binrw]
    pub struct NiStringExtraData {
        pub base: NiExtraData,
        /// The string.
        pub string_data: string,
    }
    /*
     * NiObject {
     *     name: "NiStringPalette",
     *     abstract: None,
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "List of 0x00-seperated strings, which are names of controlled objects and controller types. Used in .kf files in conjunction with NiControllerSequence.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Palette",
     *             type: "StringPalette",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "A bunch of 0x00 seperated strings.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// List of 0x00-seperated strings, which are names of controlled objects and controller types. Used in .kf files in conjunction with NiControllerSequence.
    #[binrw::binrw]
    pub struct NiStringPalette {
        pub base: NiObject,
        /// A bunch of 0x00 seperated strings.
        pub palette: StringPalette,
    }
    /*
     * NiObject {
     *     name: "NiStringsExtraData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiExtraData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Extra data in the form of a list of strings.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Num Strings",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1",
     *             ),
     *             description: Some(
     *                 "Number of strings.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Data",
     *             type: "SizedString",
     *             length: Some(
     *                 "Num Strings",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The strings.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Extra data in the form of a list of strings.
    #[binrw::binrw]
    pub struct NiStringsExtraData {
        pub base: NiExtraData,
        /// Number of strings.
        pub num_strings: u32,
        /// The strings.
        pub data: SizedString,
    }
    /*
     * NiObject {
     *     name: "NiTextKeyExtraData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiExtraData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Extra data that holds an array of NiTextKey objects for use in animation sequences.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Num Text Keys",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The number of text keys that follow.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Text Keys",
     *             type: "Key",
     *             length: Some(
     *                 "Num Text Keys",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "string",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "List of textual notes and at which time they take effect. Used for designating the start and stop of animations and the triggering of sounds.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Extra data that holds an array of NiTextKey objects for use in animation sequences.
    #[binrw::binrw]
    pub struct NiTextKeyExtraData {
        pub base: NiExtraData,
        /// The number of text keys that follow.
        pub num_text_keys: u32,
        /// List of textual notes and at which time they take effect. Used for designating the start and stop of animations and the triggering of sounds.
        pub text_keys: Key<string>,
    }
    /*
     * NiObject {
     *     name: "NiTextureEffect",
     *     abstract: None,
     *     inherit: Some(
     *         "NiDynamicEffect",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Represents an effect that uses projected textures such as projected lights (gobos), environment maps, and fog maps.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Model Projection Matrix",
     *             type: "Matrix33",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Model projection matrix.  Always identity?",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Model Projection Translation",
     *             type: "Vector3",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Model projection translation.  Always (0,0,0)?",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Texture Filtering",
     *             type: "TexFilterMode",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "FILTER_TRILERP",
     *             ),
     *             description: Some(
     *                 "Texture Filtering mode.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Max Anisotropy",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.5.0.4",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Texture Clamping",
     *             type: "TexClampMode",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "WRAP_S_WRAP_T",
     *             ),
     *             description: Some(
     *                 "Texture Clamp mode.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Texture Type",
     *             type: "TextureType",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "TEX_ENVIRONMENT_MAP",
     *             ),
     *             description: Some(
     *                 "The type of effect that the texture is used for.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Coordinate Generation Type",
     *             type: "CoordGenType",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "CG_SPHERE_MAP",
     *             ),
     *             description: Some(
     *                 "The method that will be used to generate UV coordinates for the texture effect.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Image",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "3.1",
     *             ),
     *             template: Some(
     *                 "NiImage",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Image index.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Source Texture",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "3.1",
     *             ),
     *             until: None,
     *             template: Some(
     *                 "NiSourceTexture",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Source texture index.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Enable Plane",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "0",
     *             ),
     *             description: Some(
     *                 "Determines whether a clipping plane is used. Always 8-bit.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Plane",
     *             type: "NiPlane",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "PS2 L",
     *             type: "short",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.2.0.0",
     *             ),
     *             template: None,
     *             default: Some(
     *                 "0",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "PS2 K",
     *             type: "short",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.2.0.0",
     *             ),
     *             template: None,
     *             default: Some(
     *                 "-75",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Unknown Short",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "4.1.0.12",
     *             ),
     *             template: None,
     *             default: Some(
     *                 "0",
     *             ),
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Represents an effect that uses projected textures such as projected lights (gobos), environment maps, and fog maps.
    #[binrw::binrw]
    pub struct NiTextureEffect {
        pub base: NiDynamicEffect,
        /// Model projection matrix.  Always identity?
        pub model_projection_matrix: Matrix33,
        /// Model projection translation.  Always (0,0,0)?
        pub model_projection_translation: Vector3,
        /// Texture Filtering mode.
        pub texture_filtering: TexFilterMode,
        // field is out of version range (Some("20.5.0.4"), None)
        // pub max_anisotropy: u16,
        /// Texture Clamp mode.
        pub texture_clamping: TexClampMode,
        /// The type of effect that the texture is used for.
        pub texture_type: TextureType,
        /// The method that will be used to generate UV coordinates for the texture effect.
        pub coordinate_generation_type: CoordGenType,
        // field is out of version range (None, Some("3.1"))
        // pub image: Ref<NiImage>,
        /// Source texture index.
        pub source_texture: Ref<NiSourceTexture>,
        /// Determines whether a clipping plane is used. Always 8-bit.
        pub enable_plane: u8,
        pub plane: NiPlane,
        // field is out of version range (None, Some("10.2.0.0"))
        // pub ps2_l: short,
        // field is out of version range (None, Some("10.2.0.0"))
        // pub ps2_k: short,
        // field is out of version range (None, Some("4.1.0.12"))
        // pub unknown_short: u16,
    }
    /*
     * NiObject {
     *     name: "NiTexturingProperty",
     *     abstract: None,
     *     inherit: Some(
     *         "NiProperty",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Describes how a fragment shader should be configured for a given piece of geometry.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Flags",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.0.1.2",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Property flags.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Flags",
     *             type: "TexturingFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.1.0.2",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Property flags.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Apply Mode",
     *             type: "ApplyMode",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "3.3.0.13",
     *             ),
     *             until: Some(
     *                 "20.1.0.1",
     *             ),
     *             template: None,
     *             default: Some(
     *                 "APPLY_MODULATE",
     *             ),
     *             description: Some(
     *                 "Determines how the texture will be applied.  Seems to have special functions in Oblivion.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Texture Count",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "7",
     *             ),
     *             description: Some(
     *                 "Number of textures.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Has Base Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Base Texture",
     *             type: "TexDesc",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Base Texture",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Dark Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Dark Texture",
     *             type: "TexDesc",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Dark Texture",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Detail Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Detail Texture",
     *             type: "TexDesc",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Detail Texture",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Gloss Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Gloss Texture",
     *             type: "TexDesc",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Gloss Texture",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Glow Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Glow Texture",
     *             type: "TexDesc",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Glow Texture",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Bump Map Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Texture Count #GT# 5",
     *             ),
     *             since: Some(
     *                 "3.3.0.13",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Bump Map Texture",
     *             type: "TexDesc",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Bump Map Texture",
     *             ),
     *             since: Some(
     *                 "3.3.0.13",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Bump Map Luma Scale",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Bump Map Texture",
     *             ),
     *             since: Some(
     *                 "3.3.0.13",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Bump Map Luma Offset",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Bump Map Texture",
     *             ),
     *             since: Some(
     *                 "3.3.0.13",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Bump Map Matrix",
     *             type: "Matrix22",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Bump Map Texture",
     *             ),
     *             since: Some(
     *                 "3.3.0.13",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Normal Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Texture Count #GT# 6",
     *             ),
     *             since: Some(
     *                 "20.2.0.5",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Normal Texture",
     *             type: "TexDesc",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Normal Texture",
     *             ),
     *             since: Some(
     *                 "20.2.0.5",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Parallax Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Texture Count #GT# 7",
     *             ),
     *             since: Some(
     *                 "20.2.0.5",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Parallax Texture",
     *             type: "TexDesc",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Parallax Texture",
     *             ),
     *             since: Some(
     *                 "20.2.0.5",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Parallax Offset",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Parallax Texture",
     *             ),
     *             since: Some(
     *                 "20.2.0.5",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Decal 0 Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Texture Count #GT# 6",
     *             ),
     *             since: None,
     *             until: Some(
     *                 "20.2.0.4",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Decal 0 Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Texture Count #GT# 8",
     *             ),
     *             since: Some(
     *                 "20.2.0.5",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Decal 0 Texture",
     *             type: "TexDesc",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Decal 0 Texture",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Decal 1 Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Texture Count #GT# 7",
     *             ),
     *             since: None,
     *             until: Some(
     *                 "20.2.0.4",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Decal 1 Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Texture Count #GT# 9",
     *             ),
     *             since: Some(
     *                 "20.2.0.5",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Decal 1 Texture",
     *             type: "TexDesc",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Decal 1 Texture",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Decal 2 Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Texture Count #GT# 8",
     *             ),
     *             since: None,
     *             until: Some(
     *                 "20.2.0.4",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Decal 2 Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Texture Count #GT# 10",
     *             ),
     *             since: Some(
     *                 "20.2.0.5",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Decal 2 Texture",
     *             type: "TexDesc",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Decal 2 Texture",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Decal 3 Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Texture Count #GT# 9",
     *             ),
     *             since: None,
     *             until: Some(
     *                 "20.2.0.4",
     *             ),
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Has Decal 3 Texture",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Texture Count #GT# 11",
     *             ),
     *             since: Some(
     *                 "20.2.0.5",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Decal 3 Texture",
     *             type: "TexDesc",
     *             length: None,
     *             width: None,
     *             cond: Some(
     *                 "Has Decal 3 Texture",
     *             ),
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num Shader Textures",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.0.1.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Shader Textures",
     *             type: "ShaderTexDesc",
     *             length: Some(
     *                 "Num Shader Textures",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.0.1.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Describes how a fragment shader should be configured for a given piece of geometry.
    #[binrw::binrw]
    pub struct NiTexturingProperty {
        pub base: NiProperty,
        // field is out of version range (None, Some("10.0.1.2"))
        // pub flags: u16,
        // field is out of version range (Some("20.1.0.2"), None)
        // pub flags: TexturingFlags,
        /// Determines how the texture will be applied.  Seems to have special functions in Oblivion.
        pub apply_mode: ApplyMode,
        /// Number of textures.
        pub texture_count: u32,
        pub has_base_texture: u8,
        pub base_texture: TexDesc,
        pub has_dark_texture: u8,
        pub dark_texture: TexDesc,
        pub has_detail_texture: u8,
        pub detail_texture: TexDesc,
        pub has_gloss_texture: u8,
        pub gloss_texture: TexDesc,
        pub has_glow_texture: u8,
        pub glow_texture: TexDesc,
        pub has_bump_map_texture: u8,
        pub bump_map_texture: TexDesc,
        pub bump_map_luma_scale: f32,
        pub bump_map_luma_offset: f32,
        pub bump_map_matrix: Matrix22,
        // field is out of version range (Some("20.2.0.5"), None)
        // pub has_normal_texture: u8,
        // field is out of version range (Some("20.2.0.5"), None)
        // pub normal_texture: TexDesc,
        // field is out of version range (Some("20.2.0.5"), None)
        // pub has_parallax_texture: u8,
        // field is out of version range (Some("20.2.0.5"), None)
        // pub parallax_texture: TexDesc,
        // field is out of version range (Some("20.2.0.5"), None)
        // pub parallax_offset: f32,
        pub has_decal_0_texture: u8,
        // field is out of version range (Some("20.2.0.5"), None)
        // pub has_decal_0_texture: u8,
        pub decal_0_texture: TexDesc,
        pub has_decal_1_texture: u8,
        // field is out of version range (Some("20.2.0.5"), None)
        // pub has_decal_1_texture: u8,
        pub decal_1_texture: TexDesc,
        pub has_decal_2_texture: u8,
        // field is out of version range (Some("20.2.0.5"), None)
        // pub has_decal_2_texture: u8,
        pub decal_2_texture: TexDesc,
        pub has_decal_3_texture: u8,
        // field is out of version range (Some("20.2.0.5"), None)
        // pub has_decal_3_texture: u8,
        pub decal_3_texture: TexDesc,
        pub num_shader_textures: u32,
        pub shader_textures: ShaderTexDesc,
    }
    /*
     * NiObject {
     *     name: "NiTriShape",
     *     abstract: None,
     *     inherit: Some(
     *         "NiTriBasedGeom",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "A shape node that refers to singular triangle data.",
     *     ),
     *     fields: [],
     * }
     */
    /// A shape node that refers to singular triangle data.
    #[binrw::binrw]
    pub struct NiTriShape {
        pub base: NiTriBasedGeom,
    }
    /*
     * NiObject {
     *     name: "NiTriShapeData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiTriBasedGeomData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Holds mesh data using a list of singular triangles.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Num Triangle Points",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Num Triangles times 3.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Has Triangles",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.1.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Do we have triangle data?",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Triangles",
     *             type: "Triangle",
     *             length: Some(
     *                 "Num Triangles",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.0.1.2",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Triangle data.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Triangles",
     *             type: "Triangle",
     *             length: Some(
     *                 "Num Triangles",
     *             ),
     *             width: None,
     *             cond: Some(
     *                 "Has Triangles",
     *             ),
     *             since: Some(
     *                 "10.0.1.3",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Triangle face data.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Match Groups",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "3.1",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Number of shared normals groups.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Match Groups",
     *             type: "MatchGroup",
     *             length: Some(
     *                 "Num Match Groups",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "3.1",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The shared normals.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Holds mesh data using a list of singular triangles.
    #[binrw::binrw]
    pub struct NiTriShapeData {
        pub base: NiTriBasedGeomData,
        /// Num Triangles times 3.
        pub num_triangle_points: u32,
        /// Do we have triangle data?
        pub has_triangles: u8,
        // field is out of version range (None, Some("10.0.1.2"))
        // pub triangles: Triangle,
        /// Triangle face data.
        pub triangles: Triangle,
        /// Number of shared normals groups.
        pub num_match_groups: u16,
        /// The shared normals.
        pub match_groups: MatchGroup,
    }
    /*
     * NiObject {
     *     name: "NiTriStrips",
     *     abstract: None,
     *     inherit: Some(
     *         "NiTriBasedGeom",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "A shape node that refers to data organized into strips of triangles",
     *     ),
     *     fields: [],
     * }
     */
    /// A shape node that refers to data organized into strips of triangles
    #[binrw::binrw]
    pub struct NiTriStrips {
        pub base: NiTriBasedGeom,
    }
    /*
     * NiObject {
     *     name: "NiTriStripsData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiTriBasedGeomData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Holds mesh data using strips of triangles.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Num Strips",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Strip Lengths",
     *             type: "ushort",
     *             length: Some(
     *                 "Num Strips",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The number of points in each triangle strip.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Has Points",
     *             type: "bool",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "10.0.1.3",
     *             ),
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "true",
     *             ),
     *             description: Some(
     *                 "Do we have strip point data?",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Points",
     *             type: "ushort",
     *             length: Some(
     *                 "Num Strips",
     *             ),
     *             width: Some(
     *                 "Strip Lengths",
     *             ),
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "10.0.1.2",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The points in the Triangle strips.  Size is the sum of all entries in Strip Lengths.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Points",
     *             type: "ushort",
     *             length: Some(
     *                 "Num Strips",
     *             ),
     *             width: Some(
     *                 "Strip Lengths",
     *             ),
     *             cond: Some(
     *                 "Has Points",
     *             ),
     *             since: Some(
     *                 "10.0.1.3",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The points in the Triangle strips. Size is the sum of all entries in Strip Lengths.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Holds mesh data using strips of triangles.
    #[binrw::binrw]
    pub struct NiTriStripsData {
        pub base: NiTriBasedGeomData,
        pub num_strips: u16,
        /// The number of points in each triangle strip.
        pub strip_lengths: u16,
        /// Do we have strip point data?
        pub has_points: u8,
        // field is out of version range (None, Some("10.0.1.2"))
        // pub points: u16,
        /// The points in the Triangle strips. Size is the sum of all entries in Strip Lengths.
        pub points: u16,
    }
    /*
     * NiObject {
     *     name: "NiVectorExtraData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiExtraData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "DEPRECATED (20.5).\r\n        Extra data in the form of a vector (as x, y, z, w components).",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Vector Data",
     *             type: "Vector4",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The vector data.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// DEPRECATED (20.5).
    /// Extra data in the form of a vector (as x, y, z, w components).
    #[binrw::binrw]
    pub struct NiVectorExtraData {
        pub base: NiExtraData,
        /// The vector data.
        pub vector_data: Vector4,
    }
    /*
     * NiObject {
     *     name: "NiVertexColorProperty",
     *     abstract: None,
     *     inherit: Some(
     *         "NiProperty",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Property of vertex colors. This object is referred to by the root object of the NIF file whenever some NiTriShapeData object has vertex colors with non-default settings; if not present, vertex colors have vertex_mode=2 and lighting_mode=1.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Flags",
     *             type: "VertexColorFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Vertex Mode",
     *             type: "SourceVertexMode",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "In Flags from 20.1.0.3 on.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Lighting Mode",
     *             type: "LightingMode",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "In Flags from 20.1.0.3 on.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Property of vertex colors. This object is referred to by the root object of the NIF file whenever some NiTriShapeData object has vertex colors with non-default settings; if not present, vertex colors have vertex_mode=2 and lighting_mode=1.
    #[binrw::binrw]
    pub struct NiVertexColorProperty {
        pub base: NiProperty,
        pub flags: VertexColorFlags,
        /// In Flags from 20.1.0.3 on.
        pub vertex_mode: SourceVertexMode,
        /// In Flags from 20.1.0.3 on.
        pub lighting_mode: LightingMode,
    }
    /*
     * NiObject {
     *     name: "NiVertWeightsExtraData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiExtraData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "DEPRECATED (10.x), REMOVED (?)\r\n        Not used in skinning.\r\n        Unsure of use - perhaps for morphing animation or gravity.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Num Vertices",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Number of vertices.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Weight",
     *             type: "float",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "The vertex weights.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// DEPRECATED (10.x), REMOVED (?)
    /// Not used in skinning.
    /// Unsure of use - perhaps for morphing animation or gravity.
    #[binrw::binrw]
    pub struct NiVertWeightsExtraData {
        pub base: NiExtraData,
        /// Number of vertices.
        pub num_vertices: u16,
        /// The vertex weights.
        pub weight: f32,
    }
    /*
     * NiObject {
     *     name: "NiWireframeProperty",
     *     abstract: None,
     *     inherit: Some(
     *         "NiProperty",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Allows applications to switch between drawing solid geometry or wireframe outlines.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Flags",
     *             type: "WireframeFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Allows applications to switch between drawing solid geometry or wireframe outlines.
    #[binrw::binrw]
    pub struct NiWireframeProperty {
        pub base: NiProperty,
        pub flags: WireframeFlags,
    }
    /*
     * NiObject {
     *     name: "NiZBufferProperty",
     *     abstract: None,
     *     inherit: Some(
     *         "NiProperty",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Allows applications to set the test and write modes of the renderer's Z-buffer and to set the comparison function used for the Z-buffer test.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Flags",
     *             type: "ZBufferFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "3",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Function",
     *             type: "TestFunction",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "4.1.0.12",
     *             ),
     *             until: Some(
     *                 "20.0.0.5",
     *             ),
     *             template: None,
     *             default: Some(
     *                 "TEST_LESS_EQUAL",
     *             ),
     *             description: Some(
     *                 "Z-Test function. In Flags from 20.1.0.3 on.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Allows applications to set the test and write modes of the renderer's Z-buffer and to set the comparison function used for the Z-buffer test.
    #[binrw::binrw]
    pub struct NiZBufferProperty {
        pub base: NiProperty,
        pub flags: ZBufferFlags,
        /// Z-Test function. In Flags from 20.1.0.3 on.
        pub function: TestFunction,
    }
    /*
     * NiObject {
     *     name: "NiAccumulator",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: None,
     *     fields: [],
     * }
     */
    #[binrw::binrw]
    pub struct NiAccumulator {
        pub base: NiObject,
    }
    /*
     * NiObject {
     *     name: "NiSortAdjustNode",
     *     abstract: None,
     *     inherit: Some(
     *         "NiNode",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Used to turn sorting off for individual subtrees in a scene. Useful if objects must be drawn in a fixed order.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Sorting Mode",
     *             type: "SortingMode",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "SORTING_INHERIT",
     *             ),
     *             description: Some(
     *                 "Sorting",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Accumulator",
     *             type: "Ref",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "20.0.0.3",
     *             ),
     *             template: Some(
     *                 "NiAccumulator",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// Used to turn sorting off for individual subtrees in a scene. Useful if objects must be drawn in a fixed order.
    #[binrw::binrw]
    pub struct NiSortAdjustNode {
        pub base: NiNode,
        /// Sorting
        pub sorting_mode: SortingMode,
        // field is out of version range (None, Some("20.0.0.3"))
        // pub accumulator: Ref<NiAccumulator>,
    }
    /*
     * NiObject {
     *     name: "NiLines",
     *     abstract: None,
     *     inherit: Some(
     *         "NiTriBasedGeom",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Wireframe geometry.",
     *     ),
     *     fields: [],
     * }
     */
    /// Wireframe geometry.
    #[binrw::binrw]
    pub struct NiLines {
        pub base: NiTriBasedGeom,
    }
    /*
     * NiObject {
     *     name: "NiLinesData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiGeometryData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "Wireframe geometry data.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Lines",
     *             type: "bool",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Is vertex connected to other (next?) vertex?",
     *             ),
     *         },
     *     ],
     * }
     */
    /// Wireframe geometry data.
    #[binrw::binrw]
    pub struct NiLinesData {
        pub base: NiGeometryData,
        /// Is vertex connected to other (next?) vertex?
        pub lines: u8,
    }
    /*
     * NiObject {
     *     name: "NiScreenElementsData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiTriShapeData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "DEPRECATED (20.5), functionality included in NiMeshScreenElements.\r\n        Two dimensional screen elements.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Max Polygons",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Polygons",
     *             type: "Polygon",
     *             length: Some(
     *                 "Max Polygons",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Polygon Indices",
     *             type: "ushort",
     *             length: Some(
     *                 "Max Polygons",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Polygon Grow By",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num Polygons",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Max Vertices",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Vertices Grow By",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Max Indices",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Indices Grow By",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1",
     *             ),
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// DEPRECATED (20.5), functionality included in NiMeshScreenElements.
    /// Two dimensional screen elements.
    #[binrw::binrw]
    pub struct NiScreenElementsData {
        pub base: NiTriShapeData,
        pub max_polygons: u16,
        pub polygons: Polygon,
        pub polygon_indices: u16,
        pub polygon_grow_by: u16,
        pub num_polygons: u16,
        pub max_vertices: u16,
        pub vertices_grow_by: u16,
        pub max_indices: u16,
        pub indices_grow_by: u16,
    }
    /*
     * NiObject {
     *     name: "NiScreenElements",
     *     abstract: None,
     *     inherit: Some(
     *         "NiTriShape",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "DEPRECATED (20.5), replaced by NiMeshScreenElements.\r\n        Two dimensional screen elements.",
     *     ),
     *     fields: [],
     * }
     */
    /// DEPRECATED (20.5), replaced by NiMeshScreenElements.
    /// Two dimensional screen elements.
    #[binrw::binrw]
    pub struct NiScreenElements {
        pub base: NiTriShape,
    }
    /*
     * NiObject {
     *     name: "NiRoomGroup",
     *     abstract: None,
     *     inherit: Some(
     *         "NiNode",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "NiRoomGroup represents a set of connected rooms i.e. a game level.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Shell",
     *             type: "Ptr",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiNode",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Object that represents the room group as seen from the outside.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Rooms",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Rooms",
     *             type: "Ptr",
     *             length: Some(
     *                 "Num Rooms",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiRoom",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// NiRoomGroup represents a set of connected rooms i.e. a game level.
    #[binrw::binrw]
    pub struct NiRoomGroup {
        pub base: NiNode,
        /// Object that represents the room group as seen from the outside.
        pub shell: Ptr<NiNode>,
        pub num_rooms: u32,
        pub rooms: Ptr<NiRoom>,
    }
    /*
     * NiObject {
     *     name: "NiWall",
     *     abstract: None,
     *     inherit: Some(
     *         "NiNode",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: Some(
     *         "V3_3_0_13",
     *     ),
     *     storage: None,
     *     description: None,
     *     fields: [
     *         NiObjectField {
     *             name: "Wall Plane",
     *             type: "NiPlane",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    #[binrw::binrw]
    pub struct NiWall {
        pub base: NiNode,
        pub wall_plane: NiPlane,
    }
    /*
     * NiObject {
     *     name: "NiRoom",
     *     abstract: None,
     *     inherit: Some(
     *         "NiNode",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "NiRoom objects represent cells in a cell-portal culling system.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Num Walls",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Walls",
     *             type: "Ref",
     *             length: Some(
     *                 "Num Walls",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: Some(
     *                 "3.3.0.13",
     *             ),
     *             template: Some(
     *                 "NiWall",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Wall Planes",
     *             type: "NiPlane",
     *             length: Some(
     *                 "Num Walls",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "4.0.0.0",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num In Portals",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "In Portals",
     *             type: "Ptr",
     *             length: Some(
     *                 "Num In Portals",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiPortal",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "The portals which see into the room.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Out Portals",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Out Portals",
     *             type: "Ptr",
     *             length: Some(
     *                 "Num Out Portals",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiPortal",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "The portals which see out of the room.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Fixtures",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Fixtures",
     *             type: "Ptr",
     *             length: Some(
     *                 "Num Fixtures",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiAVObject",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "All geometry associated with the room. Seems to be Ref for legacy.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// NiRoom objects represent cells in a cell-portal culling system.
    #[binrw::binrw]
    pub struct NiRoom {
        pub base: NiNode,
        pub num_walls: u32,
        // field is out of version range (None, Some("3.3.0.13"))
        // pub walls: Ref<NiWall>,
        pub wall_planes: NiPlane,
        pub num_in_portals: u32,
        /// The portals which see into the room.
        pub in_portals: Ptr<NiPortal>,
        pub num_out_portals: u32,
        /// The portals which see out of the room.
        pub out_portals: Ptr<NiPortal>,
        pub num_fixtures: u32,
        /// All geometry associated with the room. Seems to be Ref for legacy.
        pub fixtures: Ptr<NiAVObject>,
    }
    /*
     * NiObject {
     *     name: "NiPortal",
     *     abstract: None,
     *     inherit: Some(
     *         "NiAVObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "NiPortal objects are grouping nodes that support aggressive visibility culling.\r\n        They represent flat polygonal regions through which a part of a scene graph can be viewed.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Portal Flags",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Plane Count",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Unused in 20.x, possibly also 10.x.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Num Vertices",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Vertices",
     *             type: "Vector3",
     *             length: Some(
     *                 "Num Vertices",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Adjoiner",
     *             type: "Ptr",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiNode",
     *             ),
     *             default: None,
     *             description: Some(
     *                 "Root of the scenegraph which is to be seen through this portal.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// NiPortal objects are grouping nodes that support aggressive visibility culling.
    /// They represent flat polygonal regions through which a part of a scene graph can be viewed.
    #[binrw::binrw]
    pub struct NiPortal {
        pub base: NiAVObject,
        pub portal_flags: u16,
        /// Unused in 20.x, possibly also 10.x.
        pub plane_count: u16,
        pub num_vertices: u16,
        pub vertices: Vector3,
        /// Root of the scenegraph which is to be seen through this portal.
        pub adjoiner: Ptr<NiNode>,
    }
    /*
     * NiObject {
     *     name: "NiAdditionalGeometryData",
     *     abstract: None,
     *     inherit: Some(
     *         "AbstractAdditionalGeometryData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: None,
     *     fields: [
     *         NiObjectField {
     *             name: "Num Vertices",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num Block Infos",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Block Infos",
     *             type: "NiAGDDataStream",
     *             length: Some(
     *                 "Num Block Infos",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num Blocks",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Blocks",
     *             type: "NiAGDDataBlocks",
     *             length: Some(
     *                 "Num Blocks",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    #[binrw::binrw]
    pub struct NiAdditionalGeometryData {
        pub base: AbstractAdditionalGeometryData,
        pub num_vertices: u16,
        pub num_block_infos: u32,
        pub block_infos: NiAGDDataStream,
        pub num_blocks: u32,
        pub blocks: NiAGDDataBlocks,
    }
    /*
     * NiObject {
     *     name: "NiRenderObject",
     *     abstract: Some(
     *         true,
     *     ),
     *     inherit: Some(
     *         "NiAVObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "An object that can be rendered.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Material Data",
     *             type: "MaterialData",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Per-material data.",
     *             ),
     *         },
     *     ],
     * }
     */
    /// An object that can be rendered.
    #[binrw::binrw]
    pub struct NiRenderObject {
        pub base: NiAVObject,
        /// Per-material data.
        pub material_data: MaterialData,
    }
    /*
     * NiObject {
     *     name: "NiShadowGenerator",
     *     abstract: None,
     *     inherit: Some(
     *         "NiObject",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: None,
     *     until: None,
     *     storage: None,
     *     description: Some(
     *         "An NiShadowGenerator object is attached to an NiDynamicEffect object to inform the shadowing system that the effect produces shadows.",
     *     ),
     *     fields: [
     *         NiObjectField {
     *             name: "Name",
     *             type: "string",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "NiStandardShadowTechnique",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Flags",
     *             type: "NiShadowGeneratorFlags",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "0x3DB",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num Shadow Casters",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Shadow Casters",
     *             type: "Ref",
     *             length: Some(
     *                 "Num Shadow Casters",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiNode",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num Shadow Receivers",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Shadow Receivers",
     *             type: "Ref",
     *             length: Some(
     *                 "Num Shadow Receivers",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiNode",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Target",
     *             type: "Ptr",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: Some(
     *                 "NiDynamicEffect",
     *             ),
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Depth Bias",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Size Hint",
     *             type: "ushort",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: Some(
     *                 "1024",
     *             ),
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Near Clipping Distance",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.3.0.7",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Far Clipping Distance",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.3.0.7",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Directional Light Frustum Width",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: Some(
     *                 "20.3.0.7",
     *             ),
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    /// An NiShadowGenerator object is attached to an NiDynamicEffect object to inform the shadowing system that the effect produces shadows.
    #[binrw::binrw]
    pub struct NiShadowGenerator {
        pub base: NiObject,
        pub name: string,
        pub flags: NiShadowGeneratorFlags,
        pub num_shadow_casters: u32,
        pub shadow_casters: Ref<NiNode>,
        pub num_shadow_receivers: u32,
        pub shadow_receivers: Ref<NiNode>,
        pub target: Ptr<NiDynamicEffect>,
        pub depth_bias: f32,
        pub size_hint: u16,
        // field is out of version range (Some("20.3.0.7"), None)
        // pub near_clipping_distance: f32,
        // field is out of version range (Some("20.3.0.7"), None)
        // pub far_clipping_distance: f32,
        // field is out of version range (Some("20.3.0.7"), None)
        // pub directional_light_frustum_width: f32,
    }
    /*
     * NiObject {
     *     name: "NiYAMaterialProperty",
     *     abstract: None,
     *     inherit: Some(
     *         "NiProperty",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: Some(
     *         "V20_2_4_7",
     *     ),
     *     until: None,
     *     storage: None,
     *     description: None,
     *     fields: [
     *         NiObjectField {
     *             name: "Unknown Bytes 1",
     *             type: "byte",
     *             length: Some(
     *                 "14",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Unknown Float",
     *             type: "float",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Unknown Bytes 2",
     *             type: "byte",
     *             length: Some(
     *                 "13",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    #[binrw::binrw]
    pub struct NiYAMaterialProperty {
        pub base: NiProperty,
        pub unknown_bytes_1: u8,
        pub unknown_float: f32,
        pub unknown_bytes_2: u8,
    }
    /*
     * NiObject {
     *     name: "NiRimLightProperty",
     *     abstract: None,
     *     inherit: Some(
     *         "NiProperty",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: Some(
     *         "V20_2_4_7",
     *     ),
     *     until: None,
     *     storage: None,
     *     description: None,
     *     fields: [
     *         NiObjectField {
     *             name: "Unknown Byte",
     *             type: "byte",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "01 in the example nifs",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "Unknown Floats",
     *             type: "float",
     *             length: Some(
     *                 "6",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    #[binrw::binrw]
    pub struct NiRimLightProperty {
        pub base: NiProperty,
        /// 01 in the example nifs
        pub unknown_byte: u8,
        pub unknown_floats: f32,
    }
    /*
     * NiObject {
     *     name: "NiProgramLODData",
     *     abstract: None,
     *     inherit: Some(
     *         "NiLODData",
     *     ),
     *     module: Some(
     *         "NiMain",
     *     ),
     *     versions: Some(
     *         "V20_2_4_7",
     *     ),
     *     until: None,
     *     storage: None,
     *     description: None,
     *     fields: [
     *         NiObjectField {
     *             name: "Unknown Uint",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *         NiObjectField {
     *             name: "Num LOD Entries",
     *             type: "uint",
     *             length: None,
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: Some(
     *                 "Guess is that this is the number of LOD entries, but unsure given that there was only one example.",
     *             ),
     *         },
     *         NiObjectField {
     *             name: "LOD Entries",
     *             type: "QQSpeedLODEntry",
     *             length: Some(
     *                 "Num LOD Entries",
     *             ),
     *             width: None,
     *             cond: None,
     *             since: None,
     *             until: None,
     *             template: None,
     *             default: None,
     *             description: None,
     *         },
     *     ],
     * }
     */
    #[binrw::binrw]
    pub struct NiProgramLODData {
        pub base: NiLODData,
        pub unknown_uint: u32,
        /// Guess is that this is the number of LOD entries, but unsure given that there was only one example.
        pub num_lod_entries: u32,
        pub lod_entries: QQSpeedLODEntry,
    }
}
