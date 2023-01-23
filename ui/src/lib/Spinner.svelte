<div class="outer_spinner p-4 flex justify-center items-center">
	<span class="spinner" />
</div>

<style lang="sass">
.outer_spinner
    width: 100%
    height: 100%

.spinner
    width: 75px
    height: 75px
    display: inline-block
    border-width: 4px
    border-color: rgba(0, 0, 0, 0.20)
    border-top-color: #fff
    animation: spin 1s infinite linear
    border-radius: 100%
    border-style: solid

@keyframes spin
    100%
        transform: rotate(360deg)

</style>
