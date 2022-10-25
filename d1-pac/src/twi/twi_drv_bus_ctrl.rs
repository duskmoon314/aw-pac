#[doc = "Register `twi_drv_bus_ctrl` reader"]
pub struct R(crate::R<TWI_DRV_BUS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_DRV_BUS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_DRV_BUS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_DRV_BUS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `twi_drv_bus_ctrl` writer"]
pub struct W(crate::W<TWI_DRV_BUS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_DRV_BUS_CTRL_SPEC>;
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
impl From<crate::W<TWI_DRV_BUS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_DRV_BUS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sda_moe` reader - SDA manual output enable"]
pub type SDA_MOE_R = crate::BitReader<bool>;
#[doc = "Field `sda_moe` writer - SDA manual output enable"]
pub type SDA_MOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TWI_DRV_BUS_CTRL_SPEC, bool, O>;
#[doc = "Field `scl_moe` reader - SCL manual output enable"]
pub type SCL_MOE_R = crate::BitReader<bool>;
#[doc = "Field `scl_moe` writer - SCL manual output enable"]
pub type SCL_MOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TWI_DRV_BUS_CTRL_SPEC, bool, O>;
#[doc = "Field `sda_mov` reader - SDA manual output value"]
pub type SDA_MOV_R = crate::BitReader<bool>;
#[doc = "Field `sda_mov` writer - SDA manual output value"]
pub type SDA_MOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TWI_DRV_BUS_CTRL_SPEC, bool, O>;
#[doc = "Field `scl_mov` reader - SCL manual output value"]
pub type SCL_MOV_R = crate::BitReader<bool>;
#[doc = "Field `scl_mov` writer - SCL manual output value"]
pub type SCL_MOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TWI_DRV_BUS_CTRL_SPEC, bool, O>;
#[doc = "Field `sda_sta` reader - SDA current status"]
pub type SDA_STA_R = crate::BitReader<bool>;
#[doc = "Field `scl_sta` reader - SCL current status"]
pub type SCL_STA_R = crate::BitReader<bool>;
#[doc = "Field `clk_m` reader - "]
pub type CLK_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clk_m` writer - "]
pub type CLK_M_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TWI_DRV_BUS_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `clk_n` reader - "]
pub type CLK_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clk_n` writer - "]
pub type CLK_N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TWI_DRV_BUS_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `clk_duty` reader - Setting duty cycle of clock as master"]
pub type CLK_DUTY_R = crate::BitReader<CLK_DUTY_A>;
#[doc = "Setting duty cycle of clock as master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_DUTY_A {
    #[doc = "0: 50%"]
    P50 = 0,
    #[doc = "1: 40%"]
    P40 = 1,
}
impl From<CLK_DUTY_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_DUTY_A) -> Self {
        variant as u8 != 0
    }
}
impl CLK_DUTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_DUTY_A {
        match self.bits {
            false => CLK_DUTY_A::P50,
            true => CLK_DUTY_A::P40,
        }
    }
    #[doc = "Checks if the value of the field is `P50`"]
    #[inline(always)]
    pub fn is_p50(&self) -> bool {
        *self == CLK_DUTY_A::P50
    }
    #[doc = "Checks if the value of the field is `P40`"]
    #[inline(always)]
    pub fn is_p40(&self) -> bool {
        *self == CLK_DUTY_A::P40
    }
}
#[doc = "Field `clk_duty` writer - Setting duty cycle of clock as master"]
pub type CLK_DUTY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TWI_DRV_BUS_CTRL_SPEC, CLK_DUTY_A, O>;
impl<'a, const O: u8> CLK_DUTY_W<'a, O> {
    #[doc = "50%"]
    #[inline(always)]
    pub fn p50(self) -> &'a mut W {
        self.variant(CLK_DUTY_A::P50)
    }
    #[doc = "40%"]
    #[inline(always)]
    pub fn p40(self) -> &'a mut W {
        self.variant(CLK_DUTY_A::P40)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLK_COUNT_MODE_AW {
    #[doc = "0: scl clock high period count on oscl"]
    OSCL = 0,
    #[doc = "1: scl clock high period count on iscl"]
    ISCL = 1,
}
impl From<CLK_COUNT_MODE_AW> for bool {
    #[inline(always)]
    fn from(variant: CLK_COUNT_MODE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `clk_count_mode` writer - "]
pub type CLK_COUNT_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TWI_DRV_BUS_CTRL_SPEC, CLK_COUNT_MODE_AW, O>;
impl<'a, const O: u8> CLK_COUNT_MODE_W<'a, O> {
    #[doc = "scl clock high period count on oscl"]
    #[inline(always)]
    pub fn oscl(self) -> &'a mut W {
        self.variant(CLK_COUNT_MODE_AW::OSCL)
    }
    #[doc = "scl clock high period count on iscl"]
    #[inline(always)]
    pub fn iscl(self) -> &'a mut W {
        self.variant(CLK_COUNT_MODE_AW::ISCL)
    }
}
impl R {
    #[doc = "Bit 0 - SDA manual output enable"]
    #[inline(always)]
    pub fn sda_moe(&self) -> SDA_MOE_R {
        SDA_MOE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL manual output enable"]
    #[inline(always)]
    pub fn scl_moe(&self) -> SCL_MOE_R {
        SCL_MOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDA manual output value"]
    #[inline(always)]
    pub fn sda_mov(&self) -> SDA_MOV_R {
        SDA_MOV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCL manual output value"]
    #[inline(always)]
    pub fn scl_mov(&self) -> SCL_MOV_R {
        SCL_MOV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - SDA current status"]
    #[inline(always)]
    pub fn sda_sta(&self) -> SDA_STA_R {
        SDA_STA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCL current status"]
    #[inline(always)]
    pub fn scl_sta(&self) -> SCL_STA_R {
        SCL_STA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn clk_m(&self) -> CLK_M_R {
        CLK_M_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn clk_n(&self) -> CLK_N_R {
        CLK_N_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Setting duty cycle of clock as master"]
    #[inline(always)]
    pub fn clk_duty(&self) -> CLK_DUTY_R {
        CLK_DUTY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDA manual output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sda_moe(&mut self) -> SDA_MOE_W<0> {
        SDA_MOE_W::new(self)
    }
    #[doc = "Bit 1 - SCL manual output enable"]
    #[inline(always)]
    #[must_use]
    pub fn scl_moe(&mut self) -> SCL_MOE_W<1> {
        SCL_MOE_W::new(self)
    }
    #[doc = "Bit 2 - SDA manual output value"]
    #[inline(always)]
    #[must_use]
    pub fn sda_mov(&mut self) -> SDA_MOV_W<2> {
        SDA_MOV_W::new(self)
    }
    #[doc = "Bit 3 - SCL manual output value"]
    #[inline(always)]
    #[must_use]
    pub fn scl_mov(&mut self) -> SCL_MOV_W<3> {
        SCL_MOV_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn clk_m(&mut self) -> CLK_M_W<8> {
        CLK_M_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn clk_n(&mut self) -> CLK_N_W<12> {
        CLK_N_W::new(self)
    }
    #[doc = "Bit 15 - Setting duty cycle of clock as master"]
    #[inline(always)]
    #[must_use]
    pub fn clk_duty(&mut self) -> CLK_DUTY_W<15> {
        CLK_DUTY_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn clk_count_mode(&mut self) -> CLK_COUNT_MODE_W<16> {
        CLK_COUNT_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI_DRV Bus Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_drv_bus_ctrl](index.html) module"]
pub struct TWI_DRV_BUS_CTRL_SPEC;
impl crate::RegisterSpec for TWI_DRV_BUS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_drv_bus_ctrl::R](R) reader structure"]
impl crate::Readable for TWI_DRV_BUS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_drv_bus_ctrl::W](W) writer structure"]
impl crate::Writable for TWI_DRV_BUS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_drv_bus_ctrl to value 0"]
impl crate::Resettable for TWI_DRV_BUS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
