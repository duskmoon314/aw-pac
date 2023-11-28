#[doc = "Register `smhc_idst` reader"]
pub type R = crate::R<SMHC_IDST_SPEC>;
#[doc = "Register `smhc_idst` writer"]
pub type W = crate::W<SMHC_IDST_SPEC>;
#[doc = "Field `tx_int` reader - Transmit Interrupt"]
pub type TX_INT_R = crate::BitReader;
#[doc = "Field `tx_int` writer - Transmit Interrupt"]
pub type TX_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_int` reader - Receive Interrupt"]
pub type RX_INT_R = crate::BitReader;
#[doc = "Field `rx_int` writer - Receive Interrupt"]
pub type RX_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fatal_berr_int` reader - Fatal Bus Error Interrupt"]
pub type FATAL_BERR_INT_R = crate::BitReader;
#[doc = "Field `fatal_berr_int` writer - Fatal Bus Error Interrupt"]
pub type FATAL_BERR_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `des_unavl_int` reader - Descriptor Unavailable Interrupt"]
pub type DES_UNAVL_INT_R = crate::BitReader;
#[doc = "Field `des_unavl_int` writer - Descriptor Unavailable Interrupt"]
pub type DES_UNAVL_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `err_flag_sum` reader - Card Error Summary"]
pub type ERR_FLAG_SUM_R = crate::BitReader;
#[doc = "Field `err_flag_sum` writer - Card Error Summary"]
pub type ERR_FLAG_SUM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nor_int_sum` reader - Normal Interrupt Summary"]
pub type NOR_INT_SUM_R = crate::BitReader;
#[doc = "Field `nor_int_sum` writer - Normal Interrupt Summary"]
pub type NOR_INT_SUM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abn_int_sum` reader - Abnormal Interrupt Summary"]
pub type ABN_INT_SUM_R = crate::BitReader;
#[doc = "Field `abn_int_sum` writer - Abnormal Interrupt Summary"]
pub type ABN_INT_SUM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `idmac_err_sta` reader - Error Bits"]
pub type IDMAC_ERR_STA_R = crate::FieldReader<IDMAC_ERR_STA_A>;
#[doc = "Error Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDMAC_ERR_STA_A {
    #[doc = "1: Host Abort received during the transmission"]
    TRANSMISSION = 1,
    #[doc = "2: Host Abort received during the reception"]
    RECEPTION = 2,
}
impl From<IDMAC_ERR_STA_A> for u8 {
    #[inline(always)]
    fn from(variant: IDMAC_ERR_STA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDMAC_ERR_STA_A {
    type Ux = u8;
}
impl IDMAC_ERR_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IDMAC_ERR_STA_A> {
        match self.bits {
            1 => Some(IDMAC_ERR_STA_A::TRANSMISSION),
            2 => Some(IDMAC_ERR_STA_A::RECEPTION),
            _ => None,
        }
    }
    #[doc = "Host Abort received during the transmission"]
    #[inline(always)]
    pub fn is_transmission(&self) -> bool {
        *self == IDMAC_ERR_STA_A::TRANSMISSION
    }
    #[doc = "Host Abort received during the reception"]
    #[inline(always)]
    pub fn is_reception(&self) -> bool {
        *self == IDMAC_ERR_STA_A::RECEPTION
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn tx_int(&self) -> TX_INT_R {
        TX_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt"]
    #[inline(always)]
    pub fn rx_int(&self) -> RX_INT_R {
        RX_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    pub fn fatal_berr_int(&self) -> FATAL_BERR_INT_R {
        FATAL_BERR_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt"]
    #[inline(always)]
    pub fn des_unavl_int(&self) -> DES_UNAVL_INT_R {
        DES_UNAVL_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Card Error Summary"]
    #[inline(always)]
    pub fn err_flag_sum(&self) -> ERR_FLAG_SUM_R {
        ERR_FLAG_SUM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nor_int_sum(&self) -> NOR_INT_SUM_R {
        NOR_INT_SUM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn abn_int_sum(&self) -> ABN_INT_SUM_R {
        ABN_INT_SUM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Error Bits"]
    #[inline(always)]
    pub fn idmac_err_sta(&self) -> IDMAC_ERR_STA_R {
        IDMAC_ERR_STA_R::new(((self.bits >> 10) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tx_int(&mut self) -> TX_INT_W<SMHC_IDST_SPEC> {
        TX_INT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_int(&mut self) -> RX_INT_W<SMHC_IDST_SPEC> {
        RX_INT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fatal_berr_int(&mut self) -> FATAL_BERR_INT_W<SMHC_IDST_SPEC> {
        FATAL_BERR_INT_W::new(self, 2)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn des_unavl_int(&mut self) -> DES_UNAVL_INT_W<SMHC_IDST_SPEC> {
        DES_UNAVL_INT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Card Error Summary"]
    #[inline(always)]
    #[must_use]
    pub fn err_flag_sum(&mut self) -> ERR_FLAG_SUM_W<SMHC_IDST_SPEC> {
        ERR_FLAG_SUM_W::new(self, 5)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn nor_int_sum(&mut self) -> NOR_INT_SUM_W<SMHC_IDST_SPEC> {
        NOR_INT_SUM_W::new(self, 8)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn abn_int_sum(&mut self) -> ABN_INT_SUM_W<SMHC_IDST_SPEC> {
        ABN_INT_SUM_W::new(self, 9)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IDMAC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_idst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_idst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_IDST_SPEC;
impl crate::RegisterSpec for SMHC_IDST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_idst::R`](R) reader structure"]
impl crate::Readable for SMHC_IDST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_idst::W`](W) writer structure"]
impl crate::Writable for SMHC_IDST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_idst to value 0"]
impl crate::Resettable for SMHC_IDST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
