import org.w3c.dom.Document;
import org.w3c.dom.Element;
import org.w3c.dom.Node;
import org.w3c.dom.NodeList;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.transform.Transformer;
import javax.xml.transform.TransformerFactory;
import javax.xml.transform.dom.DOMSource;
import javax.xml.transform.stream.StreamResult;
import java.io.File;

public class UpdatePom {

    public static void main(String[] args) {
        try {
            String xmlFilePath = args[0];
            String propertyName = "module.group";
            String propertyValue = args[1];

            DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
            DocumentBuilder builder = factory.newDocumentBuilder();
            Document doc = builder.parse(new File(xmlFilePath));

            addProperty(doc, propertyName, propertyValue);
            saveXml(doc, xmlFilePath);
            System.out.println("Updated " + xmlFilePath);
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    private static boolean propertyExists(Document doc, String propertyName) {
        NodeList propertiesList = doc.getElementsByTagName("properties");
        if (propertiesList.getLength() > 0) {
            Node propertiesNode = propertiesList.item(0);
            NodeList properties = propertiesNode.getChildNodes();
            for (int i = 0; i < properties.getLength(); i++) {
                Node propertyNode = properties.item(i);
                if (propertyNode.getNodeType() == Node.ELEMENT_NODE &&
                        propertyName.equals(propertyNode.getNodeName())) {
                    return true;
                }
            }
        }
        return false;
    }

    private static void addProperty(Document doc, String propertyName, String propertyValue) {
        Node project = doc.getFirstChild();
        NodeList propertiesList = doc.getElementsByTagName("properties");
        if (propertiesList.getLength() > 0) {
            Node propertiesNode = propertiesList.item(0);
            Element newProperty = doc.createElement(propertyName);
            newProperty.setTextContent(propertyValue);
            propertiesNode.appendChild(newProperty);
        } else {
            // find the "dependencies" element and add the new properties before it
            Node dependencies = null;
            NodeList children = project.getChildNodes();
            for (int i = 0; i < children.getLength(); i++) {
                Node child = children.item(i);
                if (child.getNodeType() == Node.ELEMENT_NODE &&
                        "dependencies".equals(child.getNodeName())) {
                    dependencies = child;
                    break;
                }
            }
            NodeList dependenciesList = doc.getElementsByTagName("dependencies");
            // add a new properties element
            Element properties = doc.createElement("properties");
            if (dependenciesList.getLength() > 0) {
                doc.getDocumentElement().insertBefore(properties, dependencies);
            } else {
                doc.getDocumentElement().appendChild(properties);
            }
            Element newProperty = doc.createElement(propertyName);
            newProperty.setTextContent(propertyValue);
            properties.appendChild(newProperty);
        }
    }

    private static void saveXml(Document doc, String filePath) throws Exception {
        TransformerFactory transformerFactory = TransformerFactory.newInstance();
        Transformer transformer = transformerFactory.newTransformer();
        DOMSource source = new DOMSource(doc);
        StreamResult result = new StreamResult(new File(filePath));
        transformer.transform(source, result);
    }
}
