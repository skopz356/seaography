# ! [doc = " SeaORM Entity. Generated by sea-orm-codegen 0.9.1"] use sea_orm :: entity :: prelude :: * ; # [derive (Copy , Clone , Default , Debug , DeriveEntity)] pub struct Entity ; impl EntityName for Entity { fn table_name (& self) -> & str { "category" } } # [derive (Clone , Debug , PartialEq , DeriveModel , DeriveActiveModel , async_graphql :: SimpleObject , seaography_derive :: Filter)] # [sea_orm (table_name = "category")] # [graphql (complex)] # [graphql (name = "Category")] pub struct Model { pub category_id : u8 , pub name : String , pub last_update : DateTimeUtc , } # [derive (Copy , Clone , Debug , EnumIter , DeriveColumn)] pub enum Column { CategoryId , Name , LastUpdate , } # [derive (Copy , Clone , Debug , EnumIter , DerivePrimaryKey)] pub enum PrimaryKey { CategoryId , } impl PrimaryKeyTrait for PrimaryKey { type ValueType = u8 ; fn auto_increment () -> bool { true } } # [derive (Copy , Clone , Debug , EnumIter)] pub enum Relation { FilmCategory , } impl ColumnTrait for Column { type EntityName = Entity ; fn def (& self) -> ColumnDef { match self { Self :: CategoryId => ColumnType :: TinyUnsigned . def () , Self :: Name => ColumnType :: String (Some (25u32)) . def () , Self :: LastUpdate => ColumnType :: Timestamp . def () , } } } # [seaography_derive :: relation] impl RelationTrait for Relation { fn def (& self) -> RelationDef { match self { Self :: FilmCategory => Entity :: has_many (super :: film_category :: Entity) . into () , } } } impl Related < super :: film_category :: Entity > for Entity { fn to () -> RelationDef { Relation :: FilmCategory . def () } } impl ActiveModelBehavior for ActiveModel { }