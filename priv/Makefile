OTP_DIR=../otp
ERLC=$(OTP_DIR)/bin/erlc
# OTP_EMU_DIR=../otp/erts/emulator
# OTP_EMU_TEST_DIR=$(OTP_EMU_DIR)/test

compile: \
    test.S test.beam \
    test2.S test2.beam \
    ring.S ring.beam \
    mochijson.S mochijson.beam \
    bs_match_bin_SUITE.erl bs_match_bin_SUITE.beam

%.S: %.erl
	$(ERLC) -S $^

%.beam: %.erl
	$(ERLC) $^
