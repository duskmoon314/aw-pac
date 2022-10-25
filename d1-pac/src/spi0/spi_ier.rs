#[doc = "Register `spi_ier` reader"]
pub struct R(crate::R<SPI_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_ier` writer"]
pub struct W(crate::W<SPI_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_IER_SPEC>;
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
impl From<crate::W<SPI_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_rdy_int_en` reader - RXFIFO Ready Request Interrupt Enable"]
pub type RF_RDY_INT_EN_R = crate::BitReader<RF_RDY_INT_EN_A>;
#[doc = "RXFIFO Ready Request Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF_RDY_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RF_RDY_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RF_RDY_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RF_RDY_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_RDY_INT_EN_A {
        match self.bits {
            false => RF_RDY_INT_EN_A::DISABLE,
            true => RF_RDY_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RF_RDY_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RF_RDY_INT_EN_A::ENABLE
    }
}
#[doc = "Field `rf_rdy_int_en` writer - RXFIFO Ready Request Interrupt Enable"]
pub type RF_RDY_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_IER_SPEC, RF_RDY_INT_EN_A, O>;
impl<'a, const O: u8> RF_RDY_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RF_RDY_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RF_RDY_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `rf_emp_int_en` reader - RXFIFO Empty Interrupt Enable"]
pub type RF_EMP_INT_EN_R = crate::BitReader<RF_EMP_INT_EN_A>;
#[doc = "RXFIFO Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF_EMP_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RF_EMP_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RF_EMP_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RF_EMP_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_EMP_INT_EN_A {
        match self.bits {
            false => RF_EMP_INT_EN_A::DISABLE,
            true => RF_EMP_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RF_EMP_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RF_EMP_INT_EN_A::ENABLE
    }
}
#[doc = "Field `rf_emp_int_en` writer - RXFIFO Empty Interrupt Enable"]
pub type RF_EMP_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_IER_SPEC, RF_EMP_INT_EN_A, O>;
impl<'a, const O: u8> RF_EMP_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RF_EMP_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RF_EMP_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `rf_full_int_en` reader - RXFIFO Full Interrupt Enable"]
pub type RF_FULL_INT_EN_R = crate::BitReader<RF_FULL_INT_EN_A>;
#[doc = "RXFIFO Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF_FULL_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RF_FULL_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RF_FULL_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RF_FULL_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_FULL_INT_EN_A {
        match self.bits {
            false => RF_FULL_INT_EN_A::DISABLE,
            true => RF_FULL_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RF_FULL_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RF_FULL_INT_EN_A::ENABLE
    }
}
#[doc = "Field `rf_full_int_en` writer - RXFIFO Full Interrupt Enable"]
pub type RF_FULL_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_IER_SPEC, RF_FULL_INT_EN_A, O>;
impl<'a, const O: u8> RF_FULL_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RF_FULL_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RF_FULL_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `tf_erq_int_en` reader - TXFIFO Empty Request Interrupt Enable"]
pub type TF_ERQ_INT_EN_R = crate::BitReader<TF_ERQ_INT_EN_A>;
#[doc = "TXFIFO Empty Request Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF_ERQ_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TF_ERQ_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TF_ERQ_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TF_ERQ_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_ERQ_INT_EN_A {
        match self.bits {
            false => TF_ERQ_INT_EN_A::DISABLE,
            true => TF_ERQ_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TF_ERQ_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TF_ERQ_INT_EN_A::ENABLE
    }
}
#[doc = "Field `tf_erq_int_en` writer - TXFIFO Empty Request Interrupt Enable"]
pub type TF_ERQ_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_IER_SPEC, TF_ERQ_INT_EN_A, O>;
impl<'a, const O: u8> TF_ERQ_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TF_ERQ_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TF_ERQ_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `tf_emp_int_en` reader - TXFIFO Empty Interrupt Enable"]
pub type TF_EMP_INT_EN_R = crate::BitReader<TF_EMP_INT_EN_A>;
#[doc = "TXFIFO Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF_EMP_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TF_EMP_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TF_EMP_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TF_EMP_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_EMP_INT_EN_A {
        match self.bits {
            false => TF_EMP_INT_EN_A::DISABLE,
            true => TF_EMP_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TF_EMP_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TF_EMP_INT_EN_A::ENABLE
    }
}
#[doc = "Field `tf_emp_int_en` writer - TXFIFO Empty Interrupt Enable"]
pub type TF_EMP_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_IER_SPEC, TF_EMP_INT_EN_A, O>;
impl<'a, const O: u8> TF_EMP_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TF_EMP_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TF_EMP_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `tf_full_int_en` reader - TXFIFO Full Interrupt Enable"]
pub type TF_FULL_INT_EN_R = crate::BitReader<TF_FULL_INT_EN_A>;
#[doc = "TXFIFO Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF_FULL_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TF_FULL_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TF_FULL_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TF_FULL_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_FULL_INT_EN_A {
        match self.bits {
            false => TF_FULL_INT_EN_A::DISABLE,
            true => TF_FULL_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TF_FULL_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TF_FULL_INT_EN_A::ENABLE
    }
}
#[doc = "Field `tf_full_int_en` writer - TXFIFO Full Interrupt Enable"]
pub type TF_FULL_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_IER_SPEC, TF_FULL_INT_EN_A, O>;
impl<'a, const O: u8> TF_FULL_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TF_FULL_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TF_FULL_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `rf_ovf_int_en` reader - RXFIFO Overflow Interrupt Enable"]
pub type RF_OVF_INT_EN_R = crate::BitReader<RF_OVF_INT_EN_A>;
#[doc = "RXFIFO Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF_OVF_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RF_OVF_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RF_OVF_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RF_OVF_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_OVF_INT_EN_A {
        match self.bits {
            false => RF_OVF_INT_EN_A::DISABLE,
            true => RF_OVF_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RF_OVF_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RF_OVF_INT_EN_A::ENABLE
    }
}
#[doc = "Field `rf_ovf_int_en` writer - RXFIFO Overflow Interrupt Enable"]
pub type RF_OVF_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_IER_SPEC, RF_OVF_INT_EN_A, O>;
impl<'a, const O: u8> RF_OVF_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RF_OVF_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RF_OVF_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `rf_udr_int_en` reader - RXFIFO Underrun Interrupt Enable"]
pub type RF_UDR_INT_EN_R = crate::BitReader<RF_UDR_INT_EN_A>;
#[doc = "RXFIFO Underrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF_UDR_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RF_UDR_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RF_UDR_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RF_UDR_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_UDR_INT_EN_A {
        match self.bits {
            false => RF_UDR_INT_EN_A::DISABLE,
            true => RF_UDR_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RF_UDR_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RF_UDR_INT_EN_A::ENABLE
    }
}
#[doc = "Field `rf_udr_int_en` writer - RXFIFO Underrun Interrupt Enable"]
pub type RF_UDR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_IER_SPEC, RF_UDR_INT_EN_A, O>;
impl<'a, const O: u8> RF_UDR_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RF_UDR_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RF_UDR_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `tf_ovf_int_en` reader - TXFIFO Overflow Interrupt Enable"]
pub type TF_OVF_INT_EN_R = crate::BitReader<TF_OVF_INT_EN_A>;
#[doc = "TXFIFO Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF_OVF_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TF_OVF_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TF_OVF_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TF_OVF_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_OVF_INT_EN_A {
        match self.bits {
            false => TF_OVF_INT_EN_A::DISABLE,
            true => TF_OVF_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TF_OVF_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TF_OVF_INT_EN_A::ENABLE
    }
}
#[doc = "Field `tf_ovf_int_en` writer - TXFIFO Overflow Interrupt Enable"]
pub type TF_OVF_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_IER_SPEC, TF_OVF_INT_EN_A, O>;
impl<'a, const O: u8> TF_OVF_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TF_OVF_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TF_OVF_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `tf_udr_int_en` reader - TXFIFO Underrun Interrupt Enable"]
pub type TF_UDR_INT_EN_R = crate::BitReader<TF_UDR_INT_EN_A>;
#[doc = "TXFIFO Underrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF_UDR_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TF_UDR_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TF_UDR_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TF_UDR_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_UDR_INT_EN_A {
        match self.bits {
            false => TF_UDR_INT_EN_A::DISABLE,
            true => TF_UDR_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TF_UDR_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TF_UDR_INT_EN_A::ENABLE
    }
}
#[doc = "Field `tf_udr_int_en` writer - TXFIFO Underrun Interrupt Enable"]
pub type TF_UDR_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_IER_SPEC, TF_UDR_INT_EN_A, O>;
impl<'a, const O: u8> TF_UDR_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TF_UDR_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TF_UDR_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `tc_int_en` reader - Transfer Completed Interrupt Enable"]
pub type TC_INT_EN_R = crate::BitReader<TC_INT_EN_A>;
#[doc = "Transfer Completed Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TC_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TC_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TC_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC_INT_EN_A {
        match self.bits {
            false => TC_INT_EN_A::DISABLE,
            true => TC_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TC_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TC_INT_EN_A::ENABLE
    }
}
#[doc = "Field `tc_int_en` writer - Transfer Completed Interrupt Enable"]
pub type TC_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IER_SPEC, TC_INT_EN_A, O>;
impl<'a, const O: u8> TC_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TC_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TC_INT_EN_A::ENABLE)
    }
}
#[doc = "Field `ss_int_en` reader - SSI Interrupt Enable"]
pub type SS_INT_EN_R = crate::BitReader<SS_INT_EN_A>;
#[doc = "SSI Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SS_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<SS_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SS_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SS_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_INT_EN_A {
        match self.bits {
            false => SS_INT_EN_A::DISABLE,
            true => SS_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SS_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SS_INT_EN_A::ENABLE
    }
}
#[doc = "Field `ss_int_en` writer - SSI Interrupt Enable"]
pub type SS_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_IER_SPEC, SS_INT_EN_A, O>;
impl<'a, const O: u8> SS_INT_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SS_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SS_INT_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - RXFIFO Ready Request Interrupt Enable"]
    #[inline(always)]
    pub fn rf_rdy_int_en(&self) -> RF_RDY_INT_EN_R {
        RF_RDY_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXFIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub fn rf_emp_int_en(&self) -> RF_EMP_INT_EN_R {
        RF_EMP_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXFIFO Full Interrupt Enable"]
    #[inline(always)]
    pub fn rf_full_int_en(&self) -> RF_FULL_INT_EN_R {
        RF_FULL_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TXFIFO Empty Request Interrupt Enable"]
    #[inline(always)]
    pub fn tf_erq_int_en(&self) -> TF_ERQ_INT_EN_R {
        TF_ERQ_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXFIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tf_emp_int_en(&self) -> TF_EMP_INT_EN_R {
        TF_EMP_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXFIFO Full Interrupt Enable"]
    #[inline(always)]
    pub fn tf_full_int_en(&self) -> TF_FULL_INT_EN_R {
        TF_FULL_INT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - RXFIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rf_ovf_int_en(&self) -> RF_OVF_INT_EN_R {
        RF_OVF_INT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXFIFO Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn rf_udr_int_en(&self) -> RF_UDR_INT_EN_R {
        RF_UDR_INT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TXFIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn tf_ovf_int_en(&self) -> TF_OVF_INT_EN_R {
        TF_OVF_INT_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TXFIFO Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn tf_udr_int_en(&self) -> TF_UDR_INT_EN_R {
        TF_UDR_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transfer Completed Interrupt Enable"]
    #[inline(always)]
    pub fn tc_int_en(&self) -> TC_INT_EN_R {
        TC_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SSI Interrupt Enable"]
    #[inline(always)]
    pub fn ss_int_en(&self) -> SS_INT_EN_R {
        SS_INT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXFIFO Ready Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf_rdy_int_en(&mut self) -> RF_RDY_INT_EN_W<0> {
        RF_RDY_INT_EN_W::new(self)
    }
    #[doc = "Bit 1 - RXFIFO Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf_emp_int_en(&mut self) -> RF_EMP_INT_EN_W<1> {
        RF_EMP_INT_EN_W::new(self)
    }
    #[doc = "Bit 2 - RXFIFO Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf_full_int_en(&mut self) -> RF_FULL_INT_EN_W<2> {
        RF_FULL_INT_EN_W::new(self)
    }
    #[doc = "Bit 4 - TXFIFO Empty Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tf_erq_int_en(&mut self) -> TF_ERQ_INT_EN_W<4> {
        TF_ERQ_INT_EN_W::new(self)
    }
    #[doc = "Bit 5 - TXFIFO Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tf_emp_int_en(&mut self) -> TF_EMP_INT_EN_W<5> {
        TF_EMP_INT_EN_W::new(self)
    }
    #[doc = "Bit 6 - TXFIFO Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tf_full_int_en(&mut self) -> TF_FULL_INT_EN_W<6> {
        TF_FULL_INT_EN_W::new(self)
    }
    #[doc = "Bit 8 - RXFIFO Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf_ovf_int_en(&mut self) -> RF_OVF_INT_EN_W<8> {
        RF_OVF_INT_EN_W::new(self)
    }
    #[doc = "Bit 9 - RXFIFO Underrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf_udr_int_en(&mut self) -> RF_UDR_INT_EN_W<9> {
        RF_UDR_INT_EN_W::new(self)
    }
    #[doc = "Bit 10 - TXFIFO Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tf_ovf_int_en(&mut self) -> TF_OVF_INT_EN_W<10> {
        TF_OVF_INT_EN_W::new(self)
    }
    #[doc = "Bit 11 - TXFIFO Underrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tf_udr_int_en(&mut self) -> TF_UDR_INT_EN_W<11> {
        TF_UDR_INT_EN_W::new(self)
    }
    #[doc = "Bit 12 - Transfer Completed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc_int_en(&mut self) -> TC_INT_EN_W<12> {
        TC_INT_EN_W::new(self)
    }
    #[doc = "Bit 13 - SSI Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ss_int_en(&mut self) -> SS_INT_EN_W<13> {
        SS_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ier](index.html) module"]
pub struct SPI_IER_SPEC;
impl crate::RegisterSpec for SPI_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ier::R](R) reader structure"]
impl crate::Readable for SPI_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ier::W](W) writer structure"]
impl crate::Writable for SPI_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_ier to value 0"]
impl crate::Resettable for SPI_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
