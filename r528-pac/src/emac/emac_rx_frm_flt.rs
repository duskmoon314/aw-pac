#[doc = "Register `EMAC_RX_FRM_FLT` reader"]
pub struct R(crate::R<EMAC_RX_FRM_FLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_RX_FRM_FLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_RX_FRM_FLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_RX_FRM_FLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMAC_RX_FRM_FLT` writer"]
pub struct W(crate::W<EMAC_RX_FRM_FLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_RX_FRM_FLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EMAC_RX_FRM_FLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_RX_FRM_FLT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Disable Address Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_ADDR_FILTER_A {
    #[doc = "0: `0`"]
    ENABLE = 0,
    #[doc = "1: `1`"]
    DISABLE = 1,
}
impl From<DIS_ADDR_FILTER_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_ADDR_FILTER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS_ADDR_FILTER` reader - Disable Address Filter"]
pub type DIS_ADDR_FILTER_R = crate::BitReader<DIS_ADDR_FILTER_A>;
impl DIS_ADDR_FILTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_ADDR_FILTER_A {
        match self.bits {
            false => DIS_ADDR_FILTER_A::ENABLE,
            true => DIS_ADDR_FILTER_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DIS_ADDR_FILTER_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DIS_ADDR_FILTER_A::DISABLE
    }
}
#[doc = "Field `DIS_ADDR_FILTER` writer - Disable Address Filter"]
pub type DIS_ADDR_FILTER_W<'a> =
    crate::BitWriter<'a, u32, EMAC_RX_FRM_FLT_SPEC, DIS_ADDR_FILTER_A, 31>;
impl<'a> DIS_ADDR_FILTER_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DIS_ADDR_FILTER_A::ENABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DIS_ADDR_FILTER_A::DISABLE)
    }
}
#[doc = "Disable Receive Broadcast Frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_BROADCAST_A {
    #[doc = "0: `0`"]
    RECEIVE = 0,
    #[doc = "1: `1`"]
    DROP = 1,
}
impl From<DIS_BROADCAST_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_BROADCAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS_BROADCAST` reader - Disable Receive Broadcast Frames"]
pub type DIS_BROADCAST_R = crate::BitReader<DIS_BROADCAST_A>;
impl DIS_BROADCAST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_BROADCAST_A {
        match self.bits {
            false => DIS_BROADCAST_A::RECEIVE,
            true => DIS_BROADCAST_A::DROP,
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVE`"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == DIS_BROADCAST_A::RECEIVE
    }
    #[doc = "Checks if the value of the field is `DROP`"]
    #[inline(always)]
    pub fn is_drop(&self) -> bool {
        *self == DIS_BROADCAST_A::DROP
    }
}
#[doc = "Field `DIS_BROADCAST` writer - Disable Receive Broadcast Frames"]
pub type DIS_BROADCAST_W<'a> = crate::BitWriter<'a, u32, EMAC_RX_FRM_FLT_SPEC, DIS_BROADCAST_A, 17>;
impl<'a> DIS_BROADCAST_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn receive(self) -> &'a mut W {
        self.variant(DIS_BROADCAST_A::RECEIVE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn drop(self) -> &'a mut W {
        self.variant(DIS_BROADCAST_A::DROP)
    }
}
#[doc = "Receive All Multicast Frames Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ALL_MULTICAST_A {
    #[doc = "0: `0`"]
    FILTER = 0,
    #[doc = "1: `1`"]
    RECEIVE_ALL = 1,
}
impl From<RX_ALL_MULTICAST_A> for bool {
    #[inline(always)]
    fn from(variant: RX_ALL_MULTICAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_ALL_MULTICAST` reader - Receive All Multicast Frames Filter"]
pub type RX_ALL_MULTICAST_R = crate::BitReader<RX_ALL_MULTICAST_A>;
impl RX_ALL_MULTICAST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_ALL_MULTICAST_A {
        match self.bits {
            false => RX_ALL_MULTICAST_A::FILTER,
            true => RX_ALL_MULTICAST_A::RECEIVE_ALL,
        }
    }
    #[doc = "Checks if the value of the field is `FILTER`"]
    #[inline(always)]
    pub fn is_filter(&self) -> bool {
        *self == RX_ALL_MULTICAST_A::FILTER
    }
    #[doc = "Checks if the value of the field is `RECEIVE_ALL`"]
    #[inline(always)]
    pub fn is_receive_all(&self) -> bool {
        *self == RX_ALL_MULTICAST_A::RECEIVE_ALL
    }
}
#[doc = "Field `RX_ALL_MULTICAST` writer - Receive All Multicast Frames Filter"]
pub type RX_ALL_MULTICAST_W<'a> =
    crate::BitWriter<'a, u32, EMAC_RX_FRM_FLT_SPEC, RX_ALL_MULTICAST_A, 16>;
impl<'a> RX_ALL_MULTICAST_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn filter(self) -> &'a mut W {
        self.variant(RX_ALL_MULTICAST_A::FILTER)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn receive_all(self) -> &'a mut W {
        self.variant(RX_ALL_MULTICAST_A::RECEIVE_ALL)
    }
}
#[doc = "Receive Control Frames Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTL_FRM_FILTER_A {
    #[doc = "0: `0`"]
    DROP_ALL = 0,
    #[doc = "2: `10`"]
    RECEIVE_ALL = 2,
    #[doc = "3: `11`"]
    RECEIVE_ALL_WHEN_FILTER = 3,
}
impl From<CTL_FRM_FILTER_A> for u8 {
    #[inline(always)]
    fn from(variant: CTL_FRM_FILTER_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTL_FRM_FILTER` reader - Receive Control Frames Filter"]
pub type CTL_FRM_FILTER_R = crate::FieldReader<u8, CTL_FRM_FILTER_A>;
impl CTL_FRM_FILTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTL_FRM_FILTER_A> {
        match self.bits {
            0 => Some(CTL_FRM_FILTER_A::DROP_ALL),
            2 => Some(CTL_FRM_FILTER_A::RECEIVE_ALL),
            3 => Some(CTL_FRM_FILTER_A::RECEIVE_ALL_WHEN_FILTER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DROP_ALL`"]
    #[inline(always)]
    pub fn is_drop_all(&self) -> bool {
        *self == CTL_FRM_FILTER_A::DROP_ALL
    }
    #[doc = "Checks if the value of the field is `RECEIVE_ALL`"]
    #[inline(always)]
    pub fn is_receive_all(&self) -> bool {
        *self == CTL_FRM_FILTER_A::RECEIVE_ALL
    }
    #[doc = "Checks if the value of the field is `RECEIVE_ALL_WHEN_FILTER`"]
    #[inline(always)]
    pub fn is_receive_all_when_filter(&self) -> bool {
        *self == CTL_FRM_FILTER_A::RECEIVE_ALL_WHEN_FILTER
    }
}
#[doc = "Field `CTL_FRM_FILTER` writer - Receive Control Frames Filter"]
pub type CTL_FRM_FILTER_W<'a> =
    crate::FieldWriter<'a, u32, EMAC_RX_FRM_FLT_SPEC, u8, CTL_FRM_FILTER_A, 2, 12>;
impl<'a> CTL_FRM_FILTER_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn drop_all(self) -> &'a mut W {
        self.variant(CTL_FRM_FILTER_A::DROP_ALL)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn receive_all(self) -> &'a mut W {
        self.variant(CTL_FRM_FILTER_A::RECEIVE_ALL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn receive_all_when_filter(self) -> &'a mut W {
        self.variant(CTL_FRM_FILTER_A::RECEIVE_ALL_WHEN_FILTER)
    }
}
#[doc = "Filter Multicast Frames Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH_MULTICAST_A {
    #[doc = "0: `0`"]
    DA_FIELD = 0,
    #[doc = "1: `1`"]
    HASH_TABLE = 1,
}
impl From<HASH_MULTICAST_A> for bool {
    #[inline(always)]
    fn from(variant: HASH_MULTICAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASH_MULTICAST` reader - Filter Multicast Frames Set"]
pub type HASH_MULTICAST_R = crate::BitReader<HASH_MULTICAST_A>;
impl HASH_MULTICAST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH_MULTICAST_A {
        match self.bits {
            false => HASH_MULTICAST_A::DA_FIELD,
            true => HASH_MULTICAST_A::HASH_TABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DA_FIELD`"]
    #[inline(always)]
    pub fn is_da_field(&self) -> bool {
        *self == HASH_MULTICAST_A::DA_FIELD
    }
    #[doc = "Checks if the value of the field is `HASH_TABLE`"]
    #[inline(always)]
    pub fn is_hash_table(&self) -> bool {
        *self == HASH_MULTICAST_A::HASH_TABLE
    }
}
#[doc = "Field `HASH_MULTICAST` writer - Filter Multicast Frames Set"]
pub type HASH_MULTICAST_W<'a> =
    crate::BitWriter<'a, u32, EMAC_RX_FRM_FLT_SPEC, HASH_MULTICAST_A, 9>;
impl<'a> HASH_MULTICAST_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn da_field(self) -> &'a mut W {
        self.variant(HASH_MULTICAST_A::DA_FIELD)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn hash_table(self) -> &'a mut W {
        self.variant(HASH_MULTICAST_A::HASH_TABLE)
    }
}
#[doc = "Filter Unicast Frames Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH_UNICAST_A {
    #[doc = "0: `0`"]
    DA_FIELD = 0,
    #[doc = "1: `1`"]
    HASH_TABLE = 1,
}
impl From<HASH_UNICAST_A> for bool {
    #[inline(always)]
    fn from(variant: HASH_UNICAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASH_UNICAST` reader - Filter Unicast Frames Set"]
pub type HASH_UNICAST_R = crate::BitReader<HASH_UNICAST_A>;
impl HASH_UNICAST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH_UNICAST_A {
        match self.bits {
            false => HASH_UNICAST_A::DA_FIELD,
            true => HASH_UNICAST_A::HASH_TABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DA_FIELD`"]
    #[inline(always)]
    pub fn is_da_field(&self) -> bool {
        *self == HASH_UNICAST_A::DA_FIELD
    }
    #[doc = "Checks if the value of the field is `HASH_TABLE`"]
    #[inline(always)]
    pub fn is_hash_table(&self) -> bool {
        *self == HASH_UNICAST_A::HASH_TABLE
    }
}
#[doc = "Field `HASH_UNICAST` writer - Filter Unicast Frames Set"]
pub type HASH_UNICAST_W<'a> = crate::BitWriter<'a, u32, EMAC_RX_FRM_FLT_SPEC, HASH_UNICAST_A, 8>;
impl<'a> HASH_UNICAST_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn da_field(self) -> &'a mut W {
        self.variant(HASH_UNICAST_A::DA_FIELD)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn hash_table(self) -> &'a mut W {
        self.variant(HASH_UNICAST_A::HASH_TABLE)
    }
}
#[doc = "Receive SA Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SA_FILTER_EN_A {
    #[doc = "0: `0`"]
    RECEIVE_UPDATE = 0,
    #[doc = "1: `1`"]
    UPDATE_DROP_UNMATCHED = 1,
}
impl From<SA_FILTER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SA_FILTER_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SA_FILTER_EN` reader - Receive SA Filter Enable"]
pub type SA_FILTER_EN_R = crate::BitReader<SA_FILTER_EN_A>;
impl SA_FILTER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SA_FILTER_EN_A {
        match self.bits {
            false => SA_FILTER_EN_A::RECEIVE_UPDATE,
            true => SA_FILTER_EN_A::UPDATE_DROP_UNMATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVE_UPDATE`"]
    #[inline(always)]
    pub fn is_receive_update(&self) -> bool {
        *self == SA_FILTER_EN_A::RECEIVE_UPDATE
    }
    #[doc = "Checks if the value of the field is `UPDATE_DROP_UNMATCHED`"]
    #[inline(always)]
    pub fn is_update_drop_unmatched(&self) -> bool {
        *self == SA_FILTER_EN_A::UPDATE_DROP_UNMATCHED
    }
}
#[doc = "Field `SA_FILTER_EN` writer - Receive SA Filter Enable"]
pub type SA_FILTER_EN_W<'a> = crate::BitWriter<'a, u32, EMAC_RX_FRM_FLT_SPEC, SA_FILTER_EN_A, 6>;
impl<'a> SA_FILTER_EN_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn receive_update(self) -> &'a mut W {
        self.variant(SA_FILTER_EN_A::RECEIVE_UPDATE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn update_drop_unmatched(self) -> &'a mut W {
        self.variant(SA_FILTER_EN_A::UPDATE_DROP_UNMATCHED)
    }
}
#[doc = "Receive SA Invert Filter Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SA_INV_FILTER_A {
    #[doc = "0: `0`"]
    MATCHED = 0,
    #[doc = "1: `1`"]
    UNMATCHED = 1,
}
impl From<SA_INV_FILTER_A> for bool {
    #[inline(always)]
    fn from(variant: SA_INV_FILTER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SA_INV_FILTER` reader - Receive SA Invert Filter Set"]
pub type SA_INV_FILTER_R = crate::BitReader<SA_INV_FILTER_A>;
impl SA_INV_FILTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SA_INV_FILTER_A {
        match self.bits {
            false => SA_INV_FILTER_A::MATCHED,
            true => SA_INV_FILTER_A::UNMATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `MATCHED`"]
    #[inline(always)]
    pub fn is_matched(&self) -> bool {
        *self == SA_INV_FILTER_A::MATCHED
    }
    #[doc = "Checks if the value of the field is `UNMATCHED`"]
    #[inline(always)]
    pub fn is_unmatched(&self) -> bool {
        *self == SA_INV_FILTER_A::UNMATCHED
    }
}
#[doc = "Field `SA_INV_FILTER` writer - Receive SA Invert Filter Set"]
pub type SA_INV_FILTER_W<'a> = crate::BitWriter<'a, u32, EMAC_RX_FRM_FLT_SPEC, SA_INV_FILTER_A, 5>;
impl<'a> SA_INV_FILTER_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn matched(self) -> &'a mut W {
        self.variant(SA_INV_FILTER_A::MATCHED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn unmatched(self) -> &'a mut W {
        self.variant(SA_INV_FILTER_A::UNMATCHED)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DA_INV_FILTER_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    INVERSE_CMP_DA = 1,
}
impl From<DA_INV_FILTER_A> for bool {
    #[inline(always)]
    fn from(variant: DA_INV_FILTER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DA_INV_FILTER` reader - "]
pub type DA_INV_FILTER_R = crate::BitReader<DA_INV_FILTER_A>;
impl DA_INV_FILTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DA_INV_FILTER_A {
        match self.bits {
            false => DA_INV_FILTER_A::NORMAL,
            true => DA_INV_FILTER_A::INVERSE_CMP_DA,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DA_INV_FILTER_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERSE_CMP_DA`"]
    #[inline(always)]
    pub fn is_inverse_cmp_da(&self) -> bool {
        *self == DA_INV_FILTER_A::INVERSE_CMP_DA
    }
}
#[doc = "Field `DA_INV_FILTER` writer - "]
pub type DA_INV_FILTER_W<'a> = crate::BitWriter<'a, u32, EMAC_RX_FRM_FLT_SPEC, DA_INV_FILTER_A, 4>;
impl<'a> DA_INV_FILTER_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(DA_INV_FILTER_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn inverse_cmp_da(self) -> &'a mut W {
        self.variant(DA_INV_FILTER_A::INVERSE_CMP_DA)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT_MD_A {
    #[doc = "0: `0`"]
    PASSED_WHEN_MATCHED = 0,
    #[doc = "1: `1`"]
    RECEIVE_WHEN_PASSED = 1,
}
impl From<FLT_MD_A> for bool {
    #[inline(always)]
    fn from(variant: FLT_MD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT_MD` reader - "]
pub type FLT_MD_R = crate::BitReader<FLT_MD_A>;
impl FLT_MD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT_MD_A {
        match self.bits {
            false => FLT_MD_A::PASSED_WHEN_MATCHED,
            true => FLT_MD_A::RECEIVE_WHEN_PASSED,
        }
    }
    #[doc = "Checks if the value of the field is `PASSED_WHEN_MATCHED`"]
    #[inline(always)]
    pub fn is_passed_when_matched(&self) -> bool {
        *self == FLT_MD_A::PASSED_WHEN_MATCHED
    }
    #[doc = "Checks if the value of the field is `RECEIVE_WHEN_PASSED`"]
    #[inline(always)]
    pub fn is_receive_when_passed(&self) -> bool {
        *self == FLT_MD_A::RECEIVE_WHEN_PASSED
    }
}
#[doc = "Field `FLT_MD` writer - "]
pub type FLT_MD_W<'a> = crate::BitWriter<'a, u32, EMAC_RX_FRM_FLT_SPEC, FLT_MD_A, 1>;
impl<'a> FLT_MD_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn passed_when_matched(self) -> &'a mut W {
        self.variant(FLT_MD_A::PASSED_WHEN_MATCHED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn receive_when_passed(self) -> &'a mut W {
        self.variant(FLT_MD_A::RECEIVE_WHEN_PASSED)
    }
}
#[doc = "Receive All Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ALL_A {
    #[doc = "0: `0`"]
    RECEIVE_WHEN_PASSED = 0,
    #[doc = "1: `1`"]
    RECEIVE_ALL_UPDATE = 1,
}
impl From<RX_ALL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_ALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_ALL` reader - Receive All Frame"]
pub type RX_ALL_R = crate::BitReader<RX_ALL_A>;
impl RX_ALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_ALL_A {
        match self.bits {
            false => RX_ALL_A::RECEIVE_WHEN_PASSED,
            true => RX_ALL_A::RECEIVE_ALL_UPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVE_WHEN_PASSED`"]
    #[inline(always)]
    pub fn is_receive_when_passed(&self) -> bool {
        *self == RX_ALL_A::RECEIVE_WHEN_PASSED
    }
    #[doc = "Checks if the value of the field is `RECEIVE_ALL_UPDATE`"]
    #[inline(always)]
    pub fn is_receive_all_update(&self) -> bool {
        *self == RX_ALL_A::RECEIVE_ALL_UPDATE
    }
}
#[doc = "Field `RX_ALL` writer - Receive All Frame"]
pub type RX_ALL_W<'a> = crate::BitWriter<'a, u32, EMAC_RX_FRM_FLT_SPEC, RX_ALL_A, 0>;
impl<'a> RX_ALL_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn receive_when_passed(self) -> &'a mut W {
        self.variant(RX_ALL_A::RECEIVE_WHEN_PASSED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn receive_all_update(self) -> &'a mut W {
        self.variant(RX_ALL_A::RECEIVE_ALL_UPDATE)
    }
}
impl R {
    #[doc = "Bit 31 - Disable Address Filter"]
    #[inline(always)]
    pub fn dis_addr_filter(&self) -> DIS_ADDR_FILTER_R {
        DIS_ADDR_FILTER_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 17 - Disable Receive Broadcast Frames"]
    #[inline(always)]
    pub fn dis_broadcast(&self) -> DIS_BROADCAST_R {
        DIS_BROADCAST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive All Multicast Frames Filter"]
    #[inline(always)]
    pub fn rx_all_multicast(&self) -> RX_ALL_MULTICAST_R {
        RX_ALL_MULTICAST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Receive Control Frames Filter"]
    #[inline(always)]
    pub fn ctl_frm_filter(&self) -> CTL_FRM_FILTER_R {
        CTL_FRM_FILTER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 9 - Filter Multicast Frames Set"]
    #[inline(always)]
    pub fn hash_multicast(&self) -> HASH_MULTICAST_R {
        HASH_MULTICAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter Unicast Frames Set"]
    #[inline(always)]
    pub fn hash_unicast(&self) -> HASH_UNICAST_R {
        HASH_UNICAST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive SA Filter Enable"]
    #[inline(always)]
    pub fn sa_filter_en(&self) -> SA_FILTER_EN_R {
        SA_FILTER_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive SA Invert Filter Set"]
    #[inline(always)]
    pub fn sa_inv_filter(&self) -> SA_INV_FILTER_R {
        SA_INV_FILTER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn da_inv_filter(&self) -> DA_INV_FILTER_R {
        DA_INV_FILTER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn flt_md(&self) -> FLT_MD_R {
        FLT_MD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Receive All Frame"]
    #[inline(always)]
    pub fn rx_all(&self) -> RX_ALL_R {
        RX_ALL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Disable Address Filter"]
    #[inline(always)]
    pub fn dis_addr_filter(&mut self) -> DIS_ADDR_FILTER_W {
        DIS_ADDR_FILTER_W::new(self)
    }
    #[doc = "Bit 17 - Disable Receive Broadcast Frames"]
    #[inline(always)]
    pub fn dis_broadcast(&mut self) -> DIS_BROADCAST_W {
        DIS_BROADCAST_W::new(self)
    }
    #[doc = "Bit 16 - Receive All Multicast Frames Filter"]
    #[inline(always)]
    pub fn rx_all_multicast(&mut self) -> RX_ALL_MULTICAST_W {
        RX_ALL_MULTICAST_W::new(self)
    }
    #[doc = "Bits 12:13 - Receive Control Frames Filter"]
    #[inline(always)]
    pub fn ctl_frm_filter(&mut self) -> CTL_FRM_FILTER_W {
        CTL_FRM_FILTER_W::new(self)
    }
    #[doc = "Bit 9 - Filter Multicast Frames Set"]
    #[inline(always)]
    pub fn hash_multicast(&mut self) -> HASH_MULTICAST_W {
        HASH_MULTICAST_W::new(self)
    }
    #[doc = "Bit 8 - Filter Unicast Frames Set"]
    #[inline(always)]
    pub fn hash_unicast(&mut self) -> HASH_UNICAST_W {
        HASH_UNICAST_W::new(self)
    }
    #[doc = "Bit 6 - Receive SA Filter Enable"]
    #[inline(always)]
    pub fn sa_filter_en(&mut self) -> SA_FILTER_EN_W {
        SA_FILTER_EN_W::new(self)
    }
    #[doc = "Bit 5 - Receive SA Invert Filter Set"]
    #[inline(always)]
    pub fn sa_inv_filter(&mut self) -> SA_INV_FILTER_W {
        SA_INV_FILTER_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn da_inv_filter(&mut self) -> DA_INV_FILTER_W {
        DA_INV_FILTER_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn flt_md(&mut self) -> FLT_MD_W {
        FLT_MD_W::new(self)
    }
    #[doc = "Bit 0 - Receive All Frame"]
    #[inline(always)]
    pub fn rx_all(&mut self) -> RX_ALL_W {
        RX_ALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC Receive Frame Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_rx_frm_flt](index.html) module"]
pub struct EMAC_RX_FRM_FLT_SPEC;
impl crate::RegisterSpec for EMAC_RX_FRM_FLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_rx_frm_flt::R](R) reader structure"]
impl crate::Readable for EMAC_RX_FRM_FLT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_rx_frm_flt::W](W) writer structure"]
impl crate::Writable for EMAC_RX_FRM_FLT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMAC_RX_FRM_FLT to value 0"]
impl crate::Resettable for EMAC_RX_FRM_FLT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
