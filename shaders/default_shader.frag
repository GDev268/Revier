#version 450
layout (location = 0) in vec3 FragPos;
layout (location = 1) in vec3 Normal;
layout(location = 2) in vec2  FragUV;

layout (location = 0) out vec4 outColor;

layout(push_constant) uniform Push {
  mat4 modelMatrix;
  mat4 normalMatrix;
} push;

struct Material {
  vec3 ambient;
  vec3 diffuse;
  vec3 specular;    
  float shininess;
}; 

struct Light {
  vec3 position;
  vec3 color;
  vec3 rotation;  

  float intensity;
  float spot_size;
  
  float linear;
  float quadratic;

  uint type;
};

//64
layout(set = 0, binding = 1) uniform MaterialInfo {
  Material material;
  vec3 viewPos;
} object;

//64
layout(set = 0, binding = 2) uniform LightInfo {
  Light light[2];
} object_light;

layout(set = 0, binding = 3) uniform sampler2D colorMap;
layout(set = 0, binding = 4) uniform sampler2D normalMap;
layout(set = 0, binding = 5) uniform sampler2D specularMap;

vec3 CalculateDirectionalLight(Light light, vec3 normal,vec3 fragPos, vec3 viewDir);
vec3 CalculatePointLight(Light light, vec3 normal, vec3 fragPos, vec3 viewDir);
vec3 CalculateSpotLight(Light light, vec3 normal, vec3 fragPos, vec3 viewDir);

/*void main() {
  vec3 result = vec3(0.0, 0.0, 0.0);

  vec3 normal = normalize(Normal);
  vec3 viewDirection = normalize(object.viewPos - FragPos);

  /*for (int i = 0; i < object_light.lightCount; i++) {
    if (object_light.light[i].type == 0) {
      result += CalculateDirectionalLight(object_light.light[i], normalVec, FragPos, viewDirection);
    } else if (object_light.light[i].type == 1) {
      result += CalculatePointLight(object_light.light[i], normalVec, FragPos, viewDirection);
    } else if (object_light.light[i].type == 2) {
      result += CalculateSpotLight(object_light.light[i], normalVec, FragPos, viewDirection);
    }
  }

  outColor = vec4(result, 1.0);

  result += CalculateSpotLight(object_light.light,normal,FragPos,viewDirection);

  outColor = vec4(result, 1.0);
}*/

void main() {
  vec3 normal = normalize(texture(normalMap,FragUV).rgb * 2.0 - 1.0);
  vec3 viewDirection = normalize(object.viewPos - FragPos);

  vec3 result = CalculatePointLight(object_light.light[0],normal,FragPos,viewDirection);
  result += CalculateSpotLight(object_light.light[1],normal,FragPos,viewDirection);

  outColor = vec4(result, 1.0);

}


vec3 CalculateDirectionalLight(Light light,vec3 normal,vec3 fragPos,vec3 viewDirection) {
  vec3 ambient = light.color * texture(colorMap,FragUV).rgb;

  vec3 lightDirection = normalize(-light.rotation);
  float diffuseDistance = max(dot(normal,lightDirection),0.0);
  vec3 diffuse = light.color * (diffuseDistance * texture(colorMap,FragUV).rgb);

  vec3 reflectDirection = reflect(-lightDirection,normal);
  float spec = pow(max(dot(viewDirection, reflectDirection), 0.0), object.material.shininess * 128);
  vec3 specular = light.color * (spec * texture(specularMap,FragUV).rgb);
  
  return (ambient + diffuse + specular);
}

vec3 CalculatePointLight(Light light,vec3 normal,vec3 fragPos,vec3 viewDirection) {
  vec3 ambient = light.color * texture(colorMap,FragUV).rgb;
  
  vec3 lightDirection = normalize(light.position - fragPos);
  float diffuseDistance = max(dot(normal,lightDirection), 0.0);
  vec3 diffuse = light.color * (diffuseDistance * texture(colorMap,FragUV).rgb);

  vec3 reflectDirection = reflect(-lightDirection,normal);
  float spec = pow(max(dot(viewDirection, reflectDirection), 0.0), object.material.shininess * 128);
  vec3 specular = light.color * (spec * texture(specularMap,FragUV).rgb);

  float distance = length(light.position - fragPos);

  float attenuation = light.intensity / (1.0 + light.linear * distance + light.quadratic * (distance * distance));

  ambient *= attenuation;
  diffuse *= attenuation;
  specular *= attenuation;

  return (ambient + diffuse + specular);
} 

vec3 CalculateSpotLight(Light light,vec3 normal,vec3 fragPos,vec3 viewDirection) {
  vec3 ambient = light.color * texture(colorMap,FragUV).rgb;

  vec3 lightDirection = normalize(light.position - fragPos);
  float diffuseDistance = max(dot(normal,lightDirection), 0.0);
  vec3 diffuse = light.color * (diffuseDistance * texture(colorMap,FragUV).rgb);

  vec3 reflectDirection = reflect(-lightDirection,normal);
  float spec = pow(max(dot(viewDirection, reflectDirection), 0.0), object.material.shininess * 128);
  vec3 specular = light.color * (spec * texture(specularMap,FragUV).rgb);

  float theta = dot(lightDirection, normalize(-light.rotation));
  float cutOff = cos(radians(light.spot_size ));
  float outerCutOff = cos(radians(light.spot_size + 15));
  float epsilon = cutOff - outerCutOff;
  float intensity = clamp((theta - outerCutOff) / epsilon, 0.0, 1.0);

  ambient *= intensity;
  diffuse *= intensity;
  specular *= intensity;
  
  float distance = length(light.position - fragPos);

  /*float linear = 1.2833333333333333333333333333333 + ((-0.05833333333333333333333333333333) * light.range);
  float quadratic = 2.0888888888888888888888888888888 + ((-0.04074074074074074074074074074074) * light.range);*/

  float attenuation = light.intensity / (1.0 + light.linear * distance + light.quadratic * (distance * distance));

  ambient *= attenuation;
  diffuse *= attenuation;
  specular *= attenuation;

  return (ambient + diffuse + specular);
}