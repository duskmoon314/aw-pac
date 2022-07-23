#[doc = "Register `spinlock_status_reg` reader"]
pub struct R(crate::R<SPINLOCK_STATUS_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPINLOCK_STATUS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPINLOCK_STATUS_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPINLOCK_STATUS_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Lock\\[i\\]
status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_REG_STATUS_A {
    #[doc = "0: `0`"]
    FREE = 0,
    #[doc = "1: `1`"]
    TAKEN = 1,
}
impl From<LOCK_REG_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_REG_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `lock_reg(0-31)_status` reader - Lock\\[i\\]
status"]
pub type LOCK_REG_STATUS_R = crate::BitReader<LOCK_REG_STATUS_A>;
impl LOCK_REG_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_REG_STATUS_A {
        match self.bits {
            false => LOCK_REG_STATUS_A::FREE,
            true => LOCK_REG_STATUS_A::TAKEN,
        }
    }
    #[doc = "Checks if the value of the field is `FREE`"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == LOCK_REG_STATUS_A::FREE
    }
    #[doc = "Checks if the value of the field is `TAKEN`"]
    #[inline(always)]
    pub fn is_taken(&self) -> bool {
        *self == LOCK_REG_STATUS_A::TAKEN
    }
}
impl R {
    #[doc = "Lock\\[i\\]
status"]
    #[inline(always)]
    pub unsafe fn lock_reg_status(&self, n: u8) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg0_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg1_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg2_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg3_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg4_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg5_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg6_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg7_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg8_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg9_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg10_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg11_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg12_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg13_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg14_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg15_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg16_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg17_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg18_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg19_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg20_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg21_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg22_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg23_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg24_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg25_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg26_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg27_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg28_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg29_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg30_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Lock\\[i\\]
status"]
    #[inline(always)]
    pub fn lock_reg31_status(&self) -> LOCK_REG_STATUS_R {
        LOCK_REG_STATUS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Spinlock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock_status_reg](index.html) module"]
pub struct SPINLOCK_STATUS_REG_SPEC;
impl crate::RegisterSpec for SPINLOCK_STATUS_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spinlock_status_reg::R](R) reader structure"]
impl crate::Readable for SPINLOCK_STATUS_REG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets spinlock_status_reg to value 0"]
impl crate::Resettable for SPINLOCK_STATUS_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
