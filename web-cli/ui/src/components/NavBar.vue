<template>
  <div class="nav-style">
    <div class="nav">

      <div class="left-nav">
        <div style="display: flex;">
          <img src="/logo.jpeg" height="50" alt="yay">
        </div>

        <div>
          <ul>
            <li>
              <RouterLink to="/">Meta Secret</RouterLink>
            </li>
          </ul>
        </div>
      </div>

      <div class="right-nav">
        <ul>
          <li>
            <RouterLink to="/">Home</RouterLink>
          </li>
          <li>
            <RouterLink to="/about">About</RouterLink>
          </li>
          <li><a href="#">Contact</a></li>
        </ul>
      </div>

    </div>
  </div>
</template>

<style>


/* https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Flexible_Box_Layout/Aligning_Items_in_a_Flex_Container#using_auto_margins_for_main_axis_alignment */

.nav-style {
  border-bottom: solid 1px;
  border-bottom-color: #3c3c3c3b;
}

.nav {
  display: flex;
  justify-content: space-between;

  max-width: 1376px;
  margin: 0 auto;
}

.left-nav {
  display: flex;
  text-align: center;
}

img {
  margin: auto;
  display: block;
}

ul {
  padding: 10px;
  background: white;
}

.left-nav li {
  display: inline-block;
}

.right-nav li {
  display: inline-block;
  padding-left: 25px;
  padding-right: 25px;
}

a {
  display: block;
  padding: 10px;
  color: black;
  text-decoration: none;
}

</style>