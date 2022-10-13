<div class="dropdown dropdown-end dropdown-hover">
	<label tabindex="0" class="btn btn-ghost">
		<slot name="label">Dropdown</slot>
	</label>
	<ul tabindex="0" class="dropdown-content menu p-2 shadow-lg bg-base-300 rounded-box">
		<slot name="content" />
	</ul>
</div>
